//! Jobs-owned optimizer recipe and evidence names around IC Build's exact capture.
use clap::{Args, Parser};
use ic_build::artifacts::{CapturedArtifact, produce_and_retain_artifact};
use ic_host_process::status::shell_status_code;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

fn capture(source: &Path, destination: &Path) -> io::Result<()> {
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    let limit = usize::try_from(fs::metadata(source)?.len()).map_err(io::Error::other)?;
    CapturedArtifact::capture(source, parent, limit)?
        .retain_to(destination)
        .map(|_| ())
        .map_err(io::Error::other)
}

fn optimize(
    source: &Path,
    destination: &Path,
    optimizer: &OsStr,
    level: &OsStr,
) -> io::Result<ExitCode> {
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    let producer = parent.join("producer.wasm");
    // Admit the complete optimizer output, as in the original capture path.
    // Build owns exclusive producer/log admission and capture-before-removal.
    let status = produce_and_retain_artifact(
        &producer,
        &parent.join("optimizer.log"),
        destination,
        usize::MAX,
        |producer, log| {
            Command::new(optimizer)
                .arg(source)
                .arg(level)
                .arg("-o")
                .arg(producer)
                .stdout(log.try_clone()?)
                .stderr(log)
                .status()
        },
    )?;
    let code = shell_status_code(status).unwrap_or(1);
    Ok(ExitCode::from(u8::try_from(code).unwrap_or(1)))
}

#[derive(Debug, Parser)]
#[command(
    name = "jobs-capture-artifact",
    about = "Capture qualification Wasm and retain optimizer evidence",
    version
)]
struct Options {
    #[arg(value_name = "input")]
    input: PathBuf,
    #[arg(value_name = "destination")]
    destination: PathBuf,
    #[command(flatten)]
    optimization: Option<Optimization>,
}

#[derive(Args, Debug)]
#[group(requires_all = ["optimizer", "level"], multiple = true)]
struct Optimization {
    #[arg(long = "optimize", value_name = "OPTIMIZER", required = false)]
    optimizer: PathBuf,
    #[arg(value_name = "LEVEL", required = false, allow_hyphen_values = true, value_parser = ["-O3", "-Os", "-Oz"])]
    level: String,
}

fn main() -> io::Result<ExitCode> {
    let args = Options::parse();
    let status = if let Some(optimization) = args.optimization {
        optimize(
            &args.input,
            &args.destination,
            optimization.optimizer.as_os_str(),
            OsStr::new(&optimization.level),
        )?
    } else {
        capture(&args.input, &args.destination)?;
        ExitCode::SUCCESS
    };
    if status == ExitCode::SUCCESS {
        eprintln!("Retained capture: {}", args.destination.display());
    }
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use ic_host_fs::testing::FixtureDirectory;
    #[cfg(unix)]
    use ic_host_process::testing::write_executable_script;

    #[test]
    fn cli_preserves_capture_arguments_and_requires_complete_optimizer_selection() {
        let args = Options::try_parse_from(["capture", "input with spaces", "out.wasm"]).unwrap();
        assert_eq!(args.input, Path::new("input with spaces"));
        assert!(args.optimization.is_none());
        for level in ["-O3", "-Os", "-Oz"] {
            let args = Options::try_parse_from([
                "capture",
                "input",
                "output",
                "--optimize",
                "optimizer tool",
                level,
            ])
            .unwrap();
            assert_eq!(args.optimization.as_ref().unwrap().level, level);
            assert_eq!(
                args.optimization.as_ref().unwrap().optimizer,
                Path::new("optimizer tool")
            );
        }
        for args in [
            vec!["capture", "input"],
            vec!["capture", "input", "output", "-Oz"],
            vec!["capture", "input", "output", "--optimize", "optimizer"],
            vec![
                "capture",
                "input",
                "output",
                "--optimize",
                "optimizer",
                "--all-features",
            ],
        ] {
            assert!(Options::try_parse_from(args).is_err());
        }
        assert_eq!(
            Options::try_parse_from(["capture", "--help"])
                .unwrap_err()
                .kind(),
            clap::error::ErrorKind::DisplayHelp
        );
    }

    // IC Build owns generic capture/retention proofs. Keep Jobs optimizer
    // ordering here and the original-capture command path in consumer tooling.
    #[cfg(unix)]
    fn optimizer_fixture() -> (FixtureDirectory, PathBuf, PathBuf) {
        let fixture = FixtureDirectory::new(&std::env::temp_dir(), "jobs-capture").unwrap();
        let root = fixture.path();
        let source = root.join("frozen input.wasm");
        fs::write(&source, b"exact frozen bytes\n").unwrap();
        let optimizer = root.join("optimizer tool");
        write_executable_script(&optimizer, b"#!/bin/sh\n[ \"$3\" = -o ] || exit 49\ncat \"$1\" > \"$4\"\nprintf '%s\\n' \"$2\" >> \"$4\"\necho optimizer-out\necho optimizer-err >&2\n").unwrap();
        (fixture, source, optimizer)
    }

    #[cfg(unix)]
    #[test]
    fn optimized_variants_capture_exact_flags_then_retire_only_the_producer() {
        let (fixture, source, optimizer) = optimizer_fixture();
        let root = fixture.path();
        for level in ["-O3", "-Os", "-Oz"] {
            let directory = root.join(level);
            fs::create_dir(&directory).unwrap();
            let destination = directory.join("consumer.wasm");
            assert_eq!(
                optimize(
                    &source,
                    &destination,
                    optimizer.as_os_str(),
                    OsStr::new(level)
                )
                .unwrap(),
                ExitCode::SUCCESS
            );
            assert_eq!(
                fs::read(&destination).unwrap(),
                format!("exact frozen bytes\n{level}\n").as_bytes()
            );
            assert!(!directory.join("producer.wasm").exists());
            let log = fs::read_to_string(directory.join("optimizer.log")).unwrap();
            assert!(log.contains("optimizer-out") && log.contains("optimizer-err"));
            assert_eq!(fs::read_dir(&directory).unwrap().count(), 2);
        }
        assert_eq!(fs::read(&source).unwrap(), b"exact frozen bytes\n");
    }

    #[cfg(unix)]
    #[test]
    fn failed_optimization_and_failed_capture_retain_producer_and_diagnostics() {
        let (fixture, source, optimizer) = optimizer_fixture();
        let root = fixture.path();
        let failed = root.join("failed");
        fs::create_dir(&failed).unwrap();
        write_executable_script(
            &optimizer,
            b"#!/bin/sh\necho partial > \"$4\"\necho failed >&2\nexit 37\n",
        )
        .unwrap();
        assert_eq!(
            optimize(
                &source,
                &failed.join("consumer.wasm"),
                optimizer.as_os_str(),
                OsStr::new("-O3")
            )
            .unwrap(),
            ExitCode::from(37)
        );
        assert_eq!(
            fs::read(failed.join("producer.wasm")).unwrap(),
            b"partial\n"
        );
        assert_eq!(fs::read(failed.join("optimizer.log")).unwrap(), b"failed\n");
        assert!(!failed.join("consumer.wasm").exists());

        let signaled = root.join("signaled");
        fs::create_dir(&signaled).unwrap();
        write_executable_script(
            &optimizer,
            b"#!/bin/sh\necho partial > \"$4\"\necho terminated >&2\nkill -TERM $$\n",
        )
        .unwrap();
        assert_eq!(
            optimize(
                &source,
                &signaled.join("consumer.wasm"),
                optimizer.as_os_str(),
                OsStr::new("-O3")
            )
            .unwrap(),
            ExitCode::from(143)
        );
        assert_eq!(
            fs::read(signaled.join("producer.wasm")).unwrap(),
            b"partial\n"
        );
        assert_eq!(
            fs::read(signaled.join("optimizer.log")).unwrap(),
            b"terminated\n"
        );
        assert!(!signaled.join("consumer.wasm").exists());

        write_executable_script(&optimizer, b"#!/bin/sh\ncat \"$1\" > \"$4\"\n").unwrap();
        let refused = root.join("refused");
        fs::create_dir(&refused).unwrap();
        let destination = refused.join("occupied");
        fs::create_dir(&destination).unwrap();
        let error = optimize(
            &source,
            &destination,
            optimizer.as_os_str(),
            OsStr::new("-O3"),
        )
        .unwrap_err();
        let retained = error
            .get_ref()
            .unwrap()
            .downcast_ref::<ic_build::artifacts::CaptureRetentionError>()
            .unwrap();
        assert_eq!(
            fs::read(refused.join("producer.wasm")).unwrap(),
            b"exact frozen bytes\n"
        );
        assert_eq!(fs::read(&source).unwrap(), b"exact frozen bytes\n");
        assert!(refused.join("optimizer.log").is_file());
        assert_eq!(fs::read_dir(destination).unwrap().count(), 0);
        // Failure preserves its diagnostic inputs. Remove them explicitly only
        // after that proof, then require the retained copy to own the exact bytes.
        fs::remove_file(refused.join("producer.wasm")).unwrap();
        fs::remove_file(source).unwrap();
        assert_eq!(
            fs::read(&retained.retained_path).unwrap(),
            b"exact frozen bytes\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn invalid_flags_and_existing_evidence_refuse_before_optimizer_dispatch() {
        let (fixture, source, optimizer) = optimizer_fixture();
        let root = fixture.path();
        let directory = root.join("invalid");
        fs::create_dir(&directory).unwrap();
        let destination = directory.join("consumer.wasm");
        assert!(
            Options::try_parse_from([
                OsStr::new("capture"),
                source.as_os_str(),
                destination.as_os_str(),
                OsStr::new("--optimize"),
                optimizer.as_os_str(),
                OsStr::new("--all-features"),
            ])
            .is_err()
        );
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
        fs::write(directory.join("producer.wasm"), b"existing evidence").unwrap();
        assert!(
            optimize(
                &source,
                &destination,
                optimizer.as_os_str(),
                OsStr::new("-O3")
            )
            .is_err()
        );
        assert_eq!(
            fs::read(directory.join("producer.wasm")).unwrap(),
            b"existing evidence"
        );
        assert!(!directory.join("optimizer.log").exists());
    }
}
