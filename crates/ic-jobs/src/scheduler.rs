use crate::{Attempt, Job, JobError, JobRecord};

/// Due-work selection over a bounded batch of application-owned jobs.
///
/// This view borrows validated jobs without owning storage, payloads or handlers.
/// Restore persisted records through [Job::restore] before supplying them. The
/// application owns unique job identities and bounds the batch, typically using
/// its due-time index. Deadlines reported here describe only the supplied batch.
#[derive(Debug)]
pub struct Scheduler<'jobs> {
    jobs: &'jobs mut [Job],
}

impl<'jobs> Scheduler<'jobs> {
    /// Borrow a bounded batch without changing its records or ordering.
    #[must_use]
    pub fn new(jobs: &'jobs mut [Job]) -> Self {
        Self { jobs }
    }

    /// Return the earliest pending deadline in this batch, including retry backoff.
    ///
    /// Running, uncertain and terminal jobs have no dispatch deadline. Pass this
    /// deadline to the timer adapter only when the batch covers the application's
    /// earliest pending work; otherwise derive the global deadline from its index.
    #[must_use]
    pub fn next_due_ns(&self) -> Option<u64> {
        self.jobs.iter().filter_map(Job::next_due_ns).min()
    }

    /// Start at most one due attempt and return `(attempt, running_record)`.
    ///
    /// Select the earliest pending deadline, breaking ties by batch order. Return
    /// None when no pending job is due. The trusted time comes from the application.
    ///
    /// Commit the returned record with its immutable payload/index changes before
    /// dispatching an external effect. The application records the result through
    /// [Job::finish] and explicitly reconciles uncertain effects with [Job::resolve].
    /// This method never invokes a handler or schedules a platform timer.
    ///
    /// [Job::start] owns transition checks. If the selected job rejects the clock
    /// or counters, return that error without changing any job or selecting another.
    pub fn start_next(&mut self, now_ns: u64) -> Result<Option<(Attempt, JobRecord)>, JobError> {
        let Some((index, due_ns)) = self
            .jobs
            .iter()
            .enumerate()
            .filter_map(|(index, job)| job.next_due_ns().map(|due_ns| (index, due_ns)))
            .min_by_key(|&(_, due_ns)| due_ns)
        else {
            return Ok(None);
        };
        if due_ns > now_ns {
            return Ok(None);
        }
        let job = &mut self.jobs[index];
        let attempt = job.start(now_ns)?;
        Ok(Some((attempt, job.record())))
    }
}
