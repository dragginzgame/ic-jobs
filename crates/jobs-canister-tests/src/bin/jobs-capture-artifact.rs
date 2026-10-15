//! Jobs-owned optimizer recipe and evidence names around IC Build's exact capture.
use ic_build::artifacts::CapturedArtifact;
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
    let retained = CapturedArtifact::capture(source, parent, limit)?
        .retain_to(destination)
        .map_err(io::Error::other)?;
    eprintln!("Retained capture: {}", retained.display());
    Ok(())
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
    // The variant directory is caller-owned. Refuse collisions rather than
    // overwriting retained evidence from an earlier attempt.
    fs::File::create_new(&producer)?;
    let log = fs::File::create_new(parent.join("optimizer.log"))?;
    let status = Command::new(optimizer)
        .arg(source)
        .arg(level)
        .arg("-o")
        .arg(&producer)
        .stdout(log.try_clone()?)
        .stderr(log)
        .status()?;
    if !status.success() {
        // Preserve partial output, logs and the frozen input on failure.
        return Ok(optimizer_exit(status));
    }
    capture(&producer, destination)?;
    // Exact capture now owns the qualified bytes. Failure before this point
    // leaves the optimizer's producer available for diagnosis.
    fs::remove_file(producer)?;
    Ok(ExitCode::SUCCESS)
}

fn main() -> io::Result<ExitCode> {
    let args: Vec<_> = std::env::args_os().collect();
    match args.as_slice() {
        [_, source, destination] => {
            capture(Path::new(source), Path::new(destination))?;
            Ok(ExitCode::SUCCESS)
        }
        [_, source, destination, flag, optimizer, level] if flag == "--optimize" => {
            optimize(Path::new(source), Path::new(destination), optimizer, level)
        }
        _ => Err(io::Error::other(
            "usage: jobs-capture-artifact INPUT DESTINATION [--optimize OPTIMIZER {-O3|-Os|-Oz}]",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn fixture() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "jobs-capture-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        root
    }

    #[test]
    fn final_evidence_survives_producer_removal_and_leaves_no_staging_child() {
        let root = fixture();
        let source = root.join("producer");
        let destination = root.join("jobs_test_consumer.wasm");
        fs::write(&source, b"\0asm\x01\0\0\0").unwrap();
        capture(&source, &destination).unwrap();
        fs::remove_file(source).unwrap();
        assert_eq!(fs::read(destination).unwrap(), b"\0asm\x01\0\0\0");
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn failed_final_rename_preserves_exact_capture_after_producer_removal() {
        let root = fixture();
        let source = root.join("producer");
        let destination = root.join("occupied");
        fs::write(&source, b"exact failed evidence").unwrap();
        fs::create_dir(&destination).unwrap();
        assert!(capture(&source, &destination).is_err());
        fs::remove_file(source).unwrap();
        let stage = fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path != &destination)
            .unwrap();
        let retained = fs::read_dir(stage).unwrap().next().unwrap().unwrap().path();
        assert_eq!(fs::read(retained).unwrap(), b"exact failed evidence");
        assert_eq!(fs::read_dir(destination).unwrap().count(), 0);
        fs::remove_dir_all(root).unwrap();
    }
    #[cfg(unix)]
    fn optimizer_fixture() -> (PathBuf, PathBuf, PathBuf) {
        use std::os::unix::fs::PermissionsExt;
        let root = fixture();
        let source = root.join("frozen input.wasm");
        fs::write(&source, b"exact frozen bytes\n").unwrap();
        let optimizer = root.join("optimizer tool");
        fs::write(&optimizer, b"#!/bin/sh\n[ \"$3\" = -o ] || exit 49\ncat \"$1\" > \"$4\"\nprintf '%s\\n' \"$2\" >> \"$4\"\necho optimizer-out\necho optimizer-err >&2\n").unwrap();
        fs::set_permissions(&optimizer, fs::Permissions::from_mode(0o700)).unwrap();
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
        fs::write(
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

        fs::write(&optimizer, b"#!/bin/sh\ncat \"$1\" > \"$4\"\n").unwrap();
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
            fs::read(&retained.retained_path).unwrap(),
            b"exact frozen bytes\n"
        );
        assert_eq!(
            fs::read(refused.join("producer.wasm")).unwrap(),
            b"exact frozen bytes\n"
        );
        assert_eq!(fs::read(&source).unwrap(), b"exact frozen bytes\n");
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
