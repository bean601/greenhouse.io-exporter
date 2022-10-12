use serde::Deserialize;
use serde::Serialize;

#[derive(Hash, Debug)]
pub struct CandidateData {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
}

impl CandidateData {
    pub fn new(id: i64, first_name: String, last_name: String) -> CandidateData {
        CandidateData {
            id,
            first_name,
            last_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub id: i64,
    pub first_name: String,
    pub last_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    filename: String,
    url: String,
    #[serde(rename = "type")]
    attachment_type: String,
    created_at: String,
}