use super::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::io;

// Both fixture applications have tiny, bounded snapshots. Storage is not a Jobs
// trait, and a failed write keeps the previously committed bytes unchanged.
pub(crate) struct Store {
    pub(crate) bytes: Vec<u8>,
    pub(crate) writes_before_failure: Option<usize>,
}

impl Store {
    pub(crate) fn new(value: &impl Serialize) -> Result<Self> {
        let mut store = Self {
            bytes: Vec::new(),
            writes_before_failure: None,
        };
        store.write(value)?;
        Ok(store)
    }

    pub(crate) fn read<T: DeserializeOwned>(&self) -> Result<T> {
        if self.bytes.len() > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "snapshot too large").into());
        }
        Ok(serde_json::from_slice(&self.bytes)?)
    }

    pub(crate) fn write(&mut self, value: &impl Serialize) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        if bytes.len() > 4096 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "snapshot too large").into());
        }
        if let Some(remaining) = self.writes_before_failure {
            if remaining == 0 {
                self.writes_before_failure = None;
                return Err(io::Error::other("injected commit failure").into());
            }
            self.writes_before_failure = Some(remaining - 1);
        }
        self.bytes = bytes;
        Ok(())
    }
}
