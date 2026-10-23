//! Interactive host client for the real qualification canister in local PocketIC.
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use ic_platform::{
    pocket_ic::{PocketIc, PocketIcBuilder},
    principal::ExternalPrincipal,
};
use ic_testkit::pic::{
    CanisterInstallExt, InstallSpec, PocketIcInstance, PocketIcStartupConfig, advance_and_tick,
};
use ic_wire::native;
use jobs_test_consumer::{
    api::{Create, Init, Status, Summary, Task, Timing, View},
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    io::{self, BufRead, IsTerminal, Write},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

fn duration(value: &str) -> Result<Duration, String> {
    let units = [
        ('s', 1),
        ('m', 60),
        ('h', 3600),
        ('d', 86400),
        ('w', 604800),
    ];
    let (unit, factor) = units
        .into_iter()
        .find(|(unit, _)| value.ends_with(*unit))
        .ok_or("use a whole duration such as 5s, 2m, 3h, 4d or 1w")?;
    let count: u64 = value[..value.len() - unit.len_utf8()]
        .parse()
        .map_err(|_| "duration must be a nonnegative whole number followed by s, m, h, d or w")?;
    let nanos = count
        .checked_mul(factor)
        .and_then(|seconds| seconds.checked_mul(1_000_000_000))
        .ok_or("duration exceeds the canister timestamp range")?;
    Ok(Duration::from_nanos(nanos))
}

fn positive_duration(value: &str) -> Result<Duration, String> {
    let parsed = duration(value)?;
    if parsed.is_zero() {
        return Err("duration must be greater than zero".into());
    }
    Ok(parsed)
}

fn commands() -> Command {
    Command::new("queue")
        .no_binary_name(true)
        .subcommand_required(true)
        .about("Manage jobs in the local canister. Durations: 5s, 2m, 3h, 4d, 1w.")
        .after_help(
            "Clock time advances only with advance or watch. Names are single words.\n\
            Try: sample; watch 6s; advance 1m; advance 1w (one command per line).",
        )
        .subcommand(Command::new("sample").about("Queue jobs for 5s, 1m and every 1w"))
        .subcommand(
            Command::new("add")
                .about("Queue a counter increment in the canister")
                .arg(Arg::new("name").required(true))
                .arg(
                    Arg::new("after")
                        .long("after")
                        .required(true)
                        .value_parser(duration),
                )
                .arg(
                    Arg::new("every")
                        .long("every")
                        .value_parser(positive_duration),
                )
                .arg(
                    Arg::new("catch-up")
                        .long("catch-up")
                        .requires("every")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(Command::new("list").about("Read the current schedule and execution counts"))
        .subcommand(
            Command::new("advance")
                .about("Fast-forward IC time once, then deliver 32 canister rounds")
                .arg(Arg::new("duration").required(true).value_parser(duration)),
        )
        .subcommand(
            Command::new("watch")
                .about("Advance one IC second per display, waiting between displays")
                .arg(
                    Arg::new("duration")
                        .required(true)
                        .value_parser(positive_duration),
                ),
        )
        .subcommands(["pause", "resume", "cancel"].map(|name| {
            Command::new(name).arg(
                Arg::new("id")
                    .required(true)
                    .value_parser(value_parser!(u64)),
            )
        }))
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("End this temporary canister session"),
        )
}

fn failure(error: impl std::fmt::Debug) -> io::Error {
    io::Error::other(format!("{error:?}"))
}

fn span(nanos: u64) -> String {
    let mut seconds = nanos.div_ceil(1_000_000_000);
    let mut parts = vec![];
    for (unit, factor) in [
        ('w', 604800),
        ('d', 86400),
        ('h', 3600),
        ('m', 60),
        ('s', 1),
    ] {
        let count = seconds / factor;
        if count != 0 {
            parts.push(format!("{count}{unit}"));
            seconds %= factor;
        }
    }
    if parts.is_empty() {
        "0s".into()
    } else {
        parts.join(" ")
    }
}

struct Demo {
    pic: PocketIcInstance<PocketIc>,
    canister: ExternalPrincipal,
    manager: ExternalPrincipal,
    origin_ns: u64,
    // Display labels only; deadlines, state and execution counts come from RPCs.
    labels: BTreeMap<u64, (String, String)>,
}

impl Demo {
    fn new(wasm: &Path, config: PocketIcStartupConfig) -> io::Result<Self> {
        let pic = config
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .map_err(failure)?;
        let manager = ExternalPrincipal::self_authenticating(b"jobs-queue-demo-manager");
        let canister = pic
            .instance()
            .try_create_and_install(InstallSpec {
                wasm: std::fs::read(wasm)?,
                init_bytes: native::encode_request(
                    Method::Init as u64,
                    &Init {
                        manager: manager.into(),
                        delivery_sender: None,
                    },
                    REQUEST_LIMITS,
                )
                .map_err(failure)?,
                cycles: 10_000_000_000_000,
                install_sender: None,
                label: Some("jobs-queue-demo".into()),
            })
            .map_err(failure)?;
        let origin_ns = pic.instance().get_time().as_nanos_since_unix_epoch();
        println!(
            "Connected to local canister {canister} (PocketIC).\n\
            Each job increments the canister counter. This session starts fresh.\n\
            Use advance/watch to move IC time; waiting at the prompt does not move it."
        );
        Ok(Self {
            pic,
            canister,
            manager,
            origin_ns,
            labels: BTreeMap::new(),
        })
    }

    fn now(&self) -> u64 {
        self.pic.instance().get_time().as_nanos_since_unix_epoch()
    }

    fn call<T: Serialize + DeserializeOwned>(
        &self,
        method: Method,
        args: &(impl Serialize + DeserializeOwned),
        query: bool,
    ) -> io::Result<T> {
        let request =
            native::encode_request(method as u64, args, REQUEST_LIMITS).map_err(failure)?;
        let reply = if query {
            self.pic
                .instance()
                .query_call(self.canister, self.manager, method.name(), request)
        } else {
            self.pic
                .instance()
                .update_call(self.canister, self.manager, method.name(), request)
        }
        .map_err(failure)?;
        native::decode_response::<Result<T, jobs_test_consumer::api::Error>>(
            method as u64,
            &reply,
            RESPONSE_LIMITS,
        )
        .map_err(failure)?
        .map_err(failure)
    }

    fn views(&self) -> io::Result<Vec<View>> {
        let mut views = vec![];
        let mut after = None;
        loop {
            let page: Vec<View> = self.call(Method::List, &(after, 8_u8), true)?;
            let complete = page.len() < 8;
            after = page.last().map(|view| view.id);
            views.extend(page);
            if complete {
                return Ok(views);
            }
        }
    }

    fn add(
        &mut self,
        name: &str,
        after: Duration,
        every: Option<Duration>,
        catch_up: bool,
    ) -> io::Result<()> {
        let first_ns = self
            .now()
            .checked_add(u64::try_from(after.as_nanos()).map_err(failure)?)
            .ok_or_else(|| io::Error::other("deadline exceeds the canister timestamp range"))?;
        let timing = if let Some(interval) = every {
            Timing::FixedRate {
                first_ns,
                every_ns: u64::try_from(interval.as_nanos()).map_err(failure)?,
                catch_up,
            }
        } else {
            Timing::Once(first_ns)
        };
        let id: u64 = self.call(
            Method::Create,
            &(Create {
                timing,
                task: Task::Increment(1),
            },),
            false,
        )?;
        let schedule = every.map_or_else(
            || "once".into(),
            |interval| {
                format!(
                    "every {} ({})",
                    span(interval.as_nanos() as u64),
                    if catch_up { "catch-up" } else { "skip" }
                )
            },
        );
        self.labels.insert(id, (name.into(), schedule));
        println!(
            "QUEUED #{id} {name}: first run in {}",
            span(after.as_nanos() as u64)
        );
        Ok(())
    }

    fn show(&self) -> io::Result<()> {
        let summary: Summary = self.call(Method::Summary, &(), true)?;
        let now = self.now();
        println!(
            "\nIC clock +{} | counter={} | watchdog={}",
            span(now.saturating_sub(self.origin_ns) / 1_000_000_000 * 1_000_000_000),
            summary.counter,
            if summary.armed { "armed" } else { "idle" }
        );
        println!(
            "{:<4} {:<18} {:<24} {:<12} {:<20} RUNS",
            "ID", "NAME", "SCHEDULE", "STATE", "NEXT RUN"
        );
        let mut views = self.views()?;
        views.sort_by_key(|view| {
            (
                match view.status {
                    Status::Pending { due_ns } => due_ns,
                    _ => u64::MAX,
                },
                view.id,
            )
        });
        for view in views {
            let (name, schedule) = &self.labels[&view.id];
            let (state, next) = match view.status {
                Status::Pending { due_ns } => (
                    "pending",
                    if due_ns <= now {
                        "due now".into()
                    } else {
                        format!("in {}", span(due_ns - now))
                    },
                ),
                Status::Paused => ("paused", "—".into()),
                Status::Running => ("running", "—".into()),
                Status::Uncertain => ("uncertain", "—".into()),
                Status::Completed => ("completed", "—".into()),
                Status::Failed => ("failed", "—".into()),
                Status::Cancelled => ("cancelled", "—".into()),
            };
            println!(
                "{:<4} {:<18} {:<24} {:<12} {:<20} {}",
                view.id, name, schedule, state, next, view.sequence
            );
        }
        let next = summary.next_due_ns.map_or_else(
            || "none".into(),
            |due| {
                if due <= now {
                    "due now (more rounds/time may be needed for catch-up)".into()
                } else {
                    format!("in {}", span(due - now))
                }
            },
        );
        println!("Next wakeup: {next}\n");
        Ok(())
    }

    fn advance(&self, advance: Duration) -> io::Result<()> {
        self.now()
            .checked_add(u64::try_from(advance.as_nanos()).map_err(failure)?)
            .ok_or_else(|| io::Error::other("clock exceeds the canister timestamp range"))?;
        let before = self.views()?;
        // Drive real timer callbacks; never invoke dispatch_due from the host.
        // Fixed rounds keep a large CatchUp backlog from blocking the terminal.
        advance_and_tick(self.pic.instance(), advance, 32);
        for view in self.views()? {
            let previous = before
                .iter()
                .find(|entry| entry.id == view.id)
                .map_or(0, |entry| entry.sequence);
            if view.sequence > previous {
                println!(
                    "RAN #{} {}: {} execution(s), total {}",
                    view.id,
                    self.labels[&view.id].0,
                    view.sequence - previous,
                    view.sequence
                );
            }
        }
        self.show()
    }

    fn execute(&mut self, command: &ArgMatches) -> io::Result<bool> {
        let (name, args) = command.subcommand().expect("Clap requires a command");
        match name {
            "quit" => return Ok(false),
            "sample" => {
                self.add("seconds", Duration::from_secs(5), None, false)?;
                self.add("minutes", Duration::from_secs(60), None, false)?;
                self.add(
                    "weekly",
                    Duration::from_secs(604800),
                    Some(Duration::from_secs(604800)),
                    false,
                )?;
                self.show()?;
            }
            "add" => {
                self.add(
                    args.get_one::<String>("name").unwrap(),
                    *args.get_one::<Duration>("after").unwrap(),
                    args.get_one::<Duration>("every").copied(),
                    args.get_flag("catch-up"),
                )?;
                self.show()?;
            }
            "list" => self.show()?,
            "advance" => self.advance(*args.get_one::<Duration>("duration").unwrap())?,
            "watch" => {
                let mut remaining = *args.get_one::<Duration>("duration").unwrap();
                while !remaining.is_zero() && !INTERRUPTED.load(Ordering::Relaxed) {
                    let step = remaining.min(Duration::from_secs(1));
                    std::thread::sleep(step);
                    self.advance(step)?;
                    remaining -= step;
                }
            }
            "pause" | "resume" | "cancel" => {
                let method = match name {
                    "pause" => Method::Pause,
                    "resume" => Method::Resume,
                    _ => Method::Cancel,
                };
                self.call::<()>(method, &(*args.get_one::<u64>("id").unwrap(),), false)?;
                self.show()?;
            }
            _ => unreachable!("Clap owns the command set"),
        }
        Ok(true)
    }
}

fn main() -> io::Result<()> {
    let args = Command::new("queue-demo")
        .about("Watch real Jobs canister scheduling in local PocketIC")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("server")
                .long("server")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("wasm")
                .long("wasm")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
    ctrlc::set_handler(|| INTERRUPTED.store(true, Ordering::Relaxed)).map_err(failure)?;
    let config = PocketIcStartupConfig::Spawn {
        server_binary: args.get_one::<PathBuf>("server").unwrap().clone(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: Some(Duration::from_secs(3600)),
        server_idle_ttl: Some(Duration::from_secs(3600)),
        server_output_files: None,
    };
    let mut demo = Demo::new(args.get_one::<PathBuf>("wasm").unwrap(), config)?;
    commands().print_help()?;
    println!();
    let stdin = io::stdin();
    let interactive = stdin.is_terminal();
    // Read in a bounded channel so Ctrl+C can unwind the owned Testkit server
    // even while the terminal is idle. No signal-unsafe cleanup runs in a handler.
    let (sender, lines) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        for line in stdin.lock().lines() {
            if sender.send(line).is_err() {
                break;
            }
        }
    });
    loop {
        if INTERRUPTED.load(Ordering::Relaxed) {
            break;
        }
        if interactive {
            print!("jobs> ");
            io::stdout().flush()?;
        }
        let line = loop {
            if INTERRUPTED.load(Ordering::Relaxed) {
                return Ok(());
            }
            match lines.recv_timeout(Duration::from_millis(100)) {
                Ok(line) => break line,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return Ok(()),
            }
        };
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let command = match commands().try_get_matches_from(line.split_whitespace()) {
            Ok(command) => command,
            Err(error) if matches!(error.kind(), clap::error::ErrorKind::DisplayHelp) => {
                error.print()?;
                continue;
            }
            Err(error) if interactive => {
                error.print()?;
                continue;
            }
            Err(error) => return Err(failure(error)),
        };
        match demo.execute(&command) {
            Ok(false) => break,
            Ok(true) => {}
            Err(error) if interactive => eprintln!("{error}"),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_and_commands_reject_invalid_or_unbounded_input() {
        for (input, seconds) in [
            ("5s", 5),
            ("2m", 120),
            ("3h", 10800),
            ("4d", 345600),
            ("2w", 1209600),
        ] {
            assert_eq!(duration(input).unwrap(), Duration::from_secs(seconds));
        }
        for input in ["", "5", "1.5m", "-1s", "1m30s", "999999999999999999w"] {
            assert!(duration(input).is_err());
        }
        for input in [
            "add x --after 1s --every 0s",
            "add x --after 1s --catch-up",
            "watch 0s",
            "pause nope",
        ] {
            assert!(
                commands()
                    .try_get_matches_from(input.split_whitespace())
                    .is_err()
            );
        }
    }

    #[test]
    #[ignore = "explicit local canister demo: make test-demo"]
    fn canister_session_queues_seconds_minutes_weeks_and_manages_recurrence() {
        let wasm =
            std::env::var_os("IC_JOBS_CONSUMER_WASM").expect("make test-demo selects the Wasm");
        let config = PocketIcStartupConfig::from_env(Duration::from_secs(30)).unwrap();
        let mut demo = Demo::new(Path::new(&wasm), config).unwrap();
        let run = |demo: &mut Demo, line: &str| {
            let args = commands()
                .try_get_matches_from(line.split_whitespace())
                .unwrap();
            assert!(demo.execute(&args).unwrap());
        };
        run(&mut demo, "sample");
        assert!(
            demo.views()
                .unwrap()
                .iter()
                .all(|view| matches!(view.status, Status::Pending { .. }))
        );
        run(&mut demo, "advance 5s");
        assert_eq!(demo.views().unwrap()[0].status, Status::Completed);
        assert!(matches!(
            demo.views().unwrap()[1].status,
            Status::Pending { .. }
        ));
        run(&mut demo, "advance 55s");
        assert_eq!(demo.views().unwrap()[1].status, Status::Completed);
        run(&mut demo, "advance 1w");
        assert_eq!(demo.views().unwrap()[2].sequence, 1);
        run(&mut demo, "pause 3");
        run(&mut demo, "advance 2w");
        assert_eq!(demo.views().unwrap()[2].status, Status::Paused);
        assert_eq!(demo.views().unwrap()[2].sequence, 1);
        run(&mut demo, "resume 3");
        run(&mut demo, "advance 1s");
        assert_eq!(demo.views().unwrap()[2].sequence, 2);
        run(&mut demo, "cancel 3");
        run(&mut demo, "advance 1w");
        assert_eq!(demo.views().unwrap()[2].status, Status::Cancelled);
        let summary: Summary = demo.call(Method::Summary, &(), true).unwrap();
        assert_eq!(summary.counter, 4);
        assert!(!summary.armed);
    }
}
