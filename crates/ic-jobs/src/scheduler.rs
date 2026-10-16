use crate::{Attempt, Job, JobError};

/// Return the earliest pending deadline in a bounded batch, including retry backoff.
///
/// Supply validated jobs restored through [Job::restore]. Running, uncertain and
/// terminal jobs have no dispatch deadline. This deadline covers only the supplied
/// batch; the application owns any wider due-time index and timer registration.
/// Supply a finite iterator, such as `entries.iter().map(|entry| &entry.job)`.
#[must_use]
pub fn next_due_ns<'a>(jobs: impl IntoIterator<Item = &'a Job>) -> Option<u64> {
    jobs.into_iter().filter_map(Job::next_due_ns).min()
}

/// Start at most one due attempt and return `(index, attempt)`.
///
/// Supply a bounded batch of validated jobs restored through [Job::restore]. The
/// index identifies the selected job only while this iterator order is unchanged;
/// it is not a durable job identity. The application owns unique identities.
/// Select the earliest pending deadline, breaking ties by batch order. Return
/// None when no pending job is due. Trusted time comes from the application.
///
/// Commit the selected job's record with its immutable payload/index changes before
/// dispatching an external effect. Record the result through [Job::finish] and
/// explicitly reconcile uncertain effects with [Job::resolve]. This function
/// never invokes a handler or schedules a platform timer. Supply a finite iterator;
/// `entries.iter_mut().map(|entry| &mut entry.job)` keeps consumer payloads beside jobs.
///
/// [Job::start] owns transition checks. If the selected job rejects the clock
/// or counters, return that error without changing any job or selecting another.
pub fn start_next<'a>(
    jobs: impl IntoIterator<Item = &'a mut Job>,
    now_ns: u64,
) -> Result<Option<(usize, Attempt)>, JobError> {
    let Some((index, due_ns, job)) = jobs
        .into_iter()
        .enumerate()
        .filter_map(|(index, job)| job.next_due_ns().map(|due_ns| (index, due_ns, job)))
        .min_by_key(|&(_, due_ns, _)| due_ns)
    else {
        return Ok(None);
    };
    if due_ns > now_ns {
        return Ok(None);
    }
    let attempt = job.start(now_ns)?;
    Ok(Some((index, attempt)))
}
