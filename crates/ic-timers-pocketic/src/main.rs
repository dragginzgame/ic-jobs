mod build;
mod report;

fn command() -> clap::Command {
    clap::Command::new("ic-timers-pocketic")
        .about("Build and run timer qualification or exact development subjects")
        .arg(
            clap::Arg::new("worker")
                .long("worker")
                .hide(true)
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand_required(true)
        .subcommand(clap::Command::new("watchdog"))
        .subcommand(
            clap::Command::new("focused")
                .about("Run exact runtime subjects; does not replace watchdog qualification")
                .arg(clap::Arg::new("subjects").required(true).num_args(1..)),
        )
        .subcommand(
            clap::Command::new("cohorts")
                .arg(
                    clap::Arg::new("report")
                        .long("report")
                        .default_value("target/timer-cohorts.json")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                )
                .arg(
                    clap::Arg::new("baseline")
                        .long("baseline")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = command().get_matches();
    let (mode, options) = args.subcommand().expect("required subcommand");
    let qualification = match mode {
        "watchdog" => build::Qualification::Watchdog,
        "focused" => build::Qualification::Focused {
            subjects: options
                .get_many::<String>("subjects")
                .expect("required subjects")
                .cloned()
                .collect(),
        },
        "cohorts" => build::Qualification::Cohorts {
            report: options
                .get_one::<std::path::PathBuf>("report")
                .expect("default report path")
                .clone(),
            baseline: options.get_one::<std::path::PathBuf>("baseline").cloned(),
        },
        _ => unreachable!("Clap admits only declared subcommands"),
    };
    if args.get_flag("worker") {
        build::run(&qualification)
    } else {
        let mut worker = std::process::Command::new(std::env::current_exe()?);
        worker.args(["--worker", qualification.name()]);
        if let build::Qualification::Focused { subjects } = &qualification {
            worker.args(subjects);
        }
        if let build::Qualification::Cohorts { report, baseline } = &qualification {
            worker.arg("--report").arg(report);
            if let Some(baseline) = baseline {
                worker.arg("--baseline").arg(baseline);
            }
        }
        build::qualify(&mut worker, std::time::Duration::from_secs(30 * 60))?;
        Ok(())
    }
}

#[cfg(test)]
mod cohorts;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod tests;
