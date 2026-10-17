//! Jobs-owned optimizer recipe and evidence names around IC Build's exact capture.
use ic_build::artifacts::{CapturedArtifact, produce_and_retain_artifact};
use std::{
    ffi::OsStr,
    fs, io,
    path::Path,
    process::{Command, ExitCode, ExitStatus},
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

fn optimizer_exit(status: ExitStatus) -> ExitCode {
    #[cfg(unix)]
    use std::os::unix::process::ExitStatusExt;
    let code = status.code().unwrap_or_else(|| {
        #[cfg(unix)]
        {
            status.signal().map_or(1, |signal| 128 + signal)
        }
        #[cfg(not(unix))]
        {
            1
        }
    });
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}

fn optimize(
    source: &Path,
    destination: &Path,
    optimizer: &OsStr,
    level: &OsStr,
) -> io::Result<ExitCode> {
    if !["-O3", "-Os", "-Oz"]
        .iter()
        .any(|allowed| level == *allowed)
    {
        return Err(io::Error::other("expected -O3, -Os or -Oz"));
    }
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
    Ok(optimizer_exit(status))
}

fn main() -> io::Result<ExitCode> {
    let args: Vec<_> = std::env::args_os().collect();
    let (status, destination) = match args.as_slice() {
        [_, source, destination] => {
            capture(Path::new(source), Path::new(destination))?;
            (ExitCode::SUCCESS, destination)
        }
        [_, source, destination, flag, optimizer, level] if flag == "--optimize" => (
            optimize(Path::new(source), Path::new(destination), optimizer, level)?,
            destination,
        ),
        _ => {
            return Err(io::Error::other(
                "usage: jobs-capture-artifact INPUT DESTINATION [--optimize OPTIMIZER {-O3|-Os|-Oz}]",
            ));
        }
    };
    if status == ExitCode::SUCCESS {
        eprintln!("Retained capture: {}", Path::new(destination).display());
    }
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use ic_host_process::testing::write_executable_script;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        ic_host_fs::testing::create_fixture_directory(&std::env::temp_dir(), "jobs-capture")
            .unwrap()
    }

    // IC Build owns generic capture/retention proofs. Keep Jobs optimizer
    // ordering here and the original-capture command path in consumer tooling.
    #[cfg(unix)]
    fn optimizer_fixture() -> (PathBuf, PathBuf, PathBuf) {
        let root = fixture();
        let source = root.join("frozen input.wasm");
        fs::write(&source, b"exact frozen bytes\n").unwrap();
        let optimizer = root.join("optimizer tool");
        write_executable_script(&optimizer, b"#!/bin/sh\n[ \"$3\" = -o ] || exit 49\ncat \"$1\" > \"$4\"\nprintf '%s\\n' \"$2\" >> \"$4\"\necho optimizer-out\necho optimizer-err >&2\n").unwrap();
        (root, source, optimizer)
    }

    #[cfg(unix)]
    #[test]
    fn optimized_variants_capture_exact_flags_then_retire_only_the_producer() {
        let (root, source, optimizer) = optimizer_fixture();
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
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn failed_optimization_and_failed_capture_retain_producer_and_diagnostics() {
        let (root, source, optimizer) = optimizer_fixture();
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
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn invalid_flags_and_existing_evidence_refuse_before_optimizer_dispatch() {
        let (root, source, optimizer) = optimizer_fixture();
        let directory = root.join("invalid");
        fs::create_dir(&directory).unwrap();
        let destination = directory.join("consumer.wasm");
        assert!(
            optimize(
                &source,
                &destination,
                optimizer.as_os_str(),
                OsStr::new("--all-features")
            )
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
        fs::remove_dir_all(root).unwrap();
    }
}
