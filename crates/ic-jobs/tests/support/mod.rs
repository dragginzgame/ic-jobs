use ic_jobs::{Job, JobRecord};

pub(crate) fn roundtrip(job: &Job) -> Job {
    let bytes = serde_json::to_vec(&job.record()).unwrap();
    let record: JobRecord = serde_json::from_slice(&bytes).unwrap();
    Job::restore(record).unwrap()
}
