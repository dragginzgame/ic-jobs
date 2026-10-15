use crate::{Attempt, Job, JobError, JobRecord};

/// Return the earliest pending deadline in a bounded batch, including retry backoff.
///
/// Supply validated jobs restored through [Job::restore]. Running, uncertain and
/// terminal jobs have no dispatch deadline. This deadline covers only the supplied
/// batch; the application owns any wider due-time index and timer registration.
#[must_use]
pub fn next_due_ns(jobs: &[Job]) -> Option<u64> {
    jobs.iter().filter_map(Job::next_due_ns).min()
}

/// Start at most one due attempt and return `(index, attempt, running_record)`.
///
/// Supply a bounded batch of validated jobs restored through [Job::restore]. The
/// index identifies the selected job only while this batch order is unchanged;
/// it is not a durable job identity. The application owns unique identities.
/// Select the earliest pending deadline, breaking ties by batch order. Return
/// None when no pending job is due. Trusted time comes from the application.
///
/// Commit the returned record with its immutable payload/index changes before
/// dispatching an external effect. Record the result through [Job::finish] and
/// explicitly reconcile uncertain effects with [Job::resolve]. This function
/// never invokes a handler or schedules a platform timer.
///
/// [Job::start] owns transition checks. If the selected job rejects the clock
/// or counters, return that error without changing any job or selecting another.
pub fn start_next(
    jobs: &mut [Job],
    now_ns: u64,
) -> Result<Option<(usize, Attempt, JobRecord)>, JobError> {
    let Some((index, due_ns)) = jobs
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
    let job = &mut jobs[index];
    let attempt = job.start(now_ns)?;
    Ok(Some((index, attempt, job.record())))
}
