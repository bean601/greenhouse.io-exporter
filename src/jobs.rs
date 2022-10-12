use serde::Deserialize;
use serde::Serialize;

pub type Jobs = Vec<Job>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct JobData {
    pub id: i64,
    pub name: String,
}

impl JobData {
    pub fn new(id: i64, name: String) -> JobData {
        JobData { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: i64,
    pub name: String,
    pub requisition_id: String
}