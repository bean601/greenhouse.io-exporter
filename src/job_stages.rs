use serde::Deserialize;
use serde::Serialize;

pub type JobStage = Vec<JobStageElement>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct JobStageData {
    pub id: i64,
    pub name: String,
}

impl JobStageData {
    pub fn new(id: i64, name: String) -> JobStageData {
        JobStageData { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStageElement {
    pub id: i64,
    pub name: String,
    created_at: String,
    updated_at: String,
    pub job_id: i64,
}