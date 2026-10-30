//! Bounded pending one-shots, independent of IC calls and future execution.
#![forbid(unsafe_code)]

/// Admission failed before any pending task was published.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdmissionError {
    Capacity,
    IdentityExhausted,
    DeadlineOverflow,
}

pub(crate) fn deadline(now: u64, delay: std::time::Duration) -> Result<u64, AdmissionError> {
    // Backend admission deliberately combines duration and timestamp overflow.
    crate::schedule::duration_ns(delay)
        .and_then(|delay| crate::schedule::checked_deadline_after(now, delay))
        .map_err(|_| AdmissionError::DeadlineOverflow)
}

struct Pending<T> {
    id: u64,
    deadline: u64,
    submitted: bool,
    value: T,
}

/// Removal returns ownership so destructors run outside the caller's borrow.
pub(crate) struct Queue<T, const CAPACITY: usize> {
    entries: Vec<Pending<T>>,
    last_id: u64,
}

impl<T, const CAPACITY: usize> Queue<T, CAPACITY> {
    pub(crate) const fn new() -> Self {
        Self {
            entries: Vec::new(),
            last_id: 0,
        }
    }

    pub(crate) fn insert(&mut self, deadline: u64, value: T) -> Result<u64, (AdmissionError, T)> {
        if self.entries.len() == CAPACITY {
            return Err((AdmissionError::Capacity, value));
        }
        let Some(id) = self.last_id.checked_add(1) else {
            return Err((AdmissionError::IdentityExhausted, value));
        };
        self.entries.push(Pending {
            id,
            deadline,
            submitted: false,
            value,
        });
        self.last_id = id;
        Ok(id)
    }

    pub(crate) fn next(&self) -> Option<(u64, u64)> {
        self.entries
            .iter()
            .filter(|entry| !entry.submitted)
            .map(|entry| (entry.deadline, entry.id))
            .min()
    }

    pub(crate) fn submit(&mut self, id: u64) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.submitted = true;
        }
    }

    pub(crate) fn retry(&mut self, id: u64, deadline: u64) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.id == id) {
            entry.submitted = false;
            entry.deadline = deadline;
        }
    }

    pub(crate) fn remove(&mut self, id: u64) -> Option<T> {
        let index = self.entries.iter().position(|entry| entry.id == id)?;
        Some(self.entries.swap_remove(index).value)
    }
}

#[cfg(test)]
mod tests;
