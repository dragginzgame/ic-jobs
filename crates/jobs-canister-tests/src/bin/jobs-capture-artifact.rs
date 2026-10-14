//! Jobs-owned evidence filenames and custody around IC Build's exact capture.
use ic_build::artifacts::CapturedArtifact;
use std::{fs, io, path::Path};

fn capture(source: &Path, destination: &Path) -> io::Result<()> {
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    let limit = usize::try_from(fs::metadata(source)?.len()).map_err(io::Error::other)?;
    let retained = CapturedArtifact::capture(source, parent, limit)?.retain();
    // Record custody before rename: any failure leaves these exact bytes retained.
    eprintln!("Retained capture: {}", retained.display());
    fs::rename(&retained, destination)?;
    fs::remove_dir(
        retained
            .parent()
            .ok_or_else(|| io::Error::other("missing staging parent"))?,
    )
}

fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        return Err(io::Error::other(
            "usage: jobs-capture-artifact INPUT DESTINATION",
        ));
    }
    capture(Path::new(&args[1]), Path::new(&args[2]))
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
}
