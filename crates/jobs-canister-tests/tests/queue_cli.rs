//! Explicit process-level qualification of the real terminal and owned server.
#![cfg(unix)]

use std::{
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

struct Session {
    child: Child,
    server_pid: PathBuf,
}

fn signal(pid: &str, signal: &str) -> bool {
    Command::new("/bin/kill")
        .args([signal, pid])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("invoke kill")
        .success()
}

impl Drop for Session {
    fn drop(&mut self) {
        // Failure containment only: assertions below observe natural cleanup first.
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Ok(pid) = fs::read_to_string(&self.server_pid) {
            signal(pid.trim(), "-KILL");
        }
    }
}

fn until(mut condition: impl FnMut() -> bool, description: &str) {
    let deadline = Instant::now() + Duration::from_secs(45);
    while !condition() {
        assert!(Instant::now() < deadline, "timed out: {description}");
        thread::sleep(Duration::from_millis(50));
    }
}

#[test]
#[ignore = "explicit real CLI/server shutdown: make test-demo-cli"]
fn quit_eof_and_idle_interrupt_release_owned_server() {
    let binary = std::env::var_os("IC_JOBS_QUEUE_BIN").expect("queue binary");
    let server = std::env::var_os("IC_JOBS_QUEUE_SERVER").expect("admitted server binary");
    let wasm = std::env::var_os("IC_JOBS_CONSUMER_WASM").expect("consumer Wasm");
    for mode in ["quit", "eof", "interrupt", "watch-interrupt"] {
        let fixture =
            ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "jobs-queue-cli")
                .unwrap();
        let root = fixture.path();
        let wrapper = root.join("server");
        // exec preserves the PID while leaving all server arguments untouched.
        fs::write(&wrapper, "#!/bin/sh\nprintf '%s\\n' \"$$\" > \"$JOBS_SERVER_PID\"\nexec \"$JOBS_REAL_SERVER\" \"$@\"\n").unwrap();
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
        let log_path = root.join("terminal.log");
        let log = fs::File::create(&log_path).unwrap();
        let server_pid = root.join("server.pid");
        let child = Command::new(&binary)
            .arg("--server")
            .arg(&wrapper)
            .arg("--wasm")
            .arg(&wasm)
            .env("JOBS_REAL_SERVER", &server)
            .env("JOBS_SERVER_PID", &server_pid)
            .stdin(Stdio::piped())
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap();
        let mut session = Session { child, server_pid };
        let mut stdin = session.child.stdin.take().unwrap();
        writeln!(stdin, "list").unwrap();
        until(
            || {
                let output = fs::read_to_string(&log_path).unwrap();
                assert!(
                    session.child.try_wait().unwrap().is_none(),
                    "{mode}: {output}"
                );
                output.contains("Next wakeup:")
            },
            "canister installed and terminal accepting commands",
        );
        let pid = fs::read_to_string(&session.server_pid).unwrap();
        assert!(signal(pid.trim(), "-0"), "server must be running");
        match mode {
            "quit" => writeln!(stdin, "quit").unwrap(),
            "eof" => drop(stdin),
            "interrupt" => assert!(signal(&session.child.id().to_string(), "-INT")),
            "watch-interrupt" => {
                writeln!(stdin, "add after-stop --after 1s\nwatch 1w").unwrap();
                until(
                    || fs::read_to_string(&log_path).unwrap().contains("Watching;"),
                    "watch entered before its first advance",
                );
                assert!(signal(&session.child.id().to_string(), "-INT"));
            }
            _ => unreachable!(),
        }
        until(
            || session.child.try_wait().unwrap().is_some(),
            "terminal exit",
        );
        assert!(
            session.child.wait().unwrap().success(),
            "{mode}: {}",
            fs::read_to_string(&log_path).unwrap()
        );
        until(|| !signal(pid.trim(), "-0"), "owned server exit");
        if mode == "watch-interrupt" {
            assert!(
                !fs::read_to_string(&log_path).unwrap().contains("RAN #"),
                "watch dispatched work after Ctrl+C"
            );
        }
        // Do not send cleanup signals after successful natural termination.
        fs::remove_file(&session.server_pid).unwrap();
    }
}
