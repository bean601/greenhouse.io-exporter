use crate::candidates::CandidateData;
use serde::Deserialize;
use serde::Serialize;

pub type Applications = Vec<Application>;

#[derive(Hash, Debug)]
pub struct AttachmentDownload {
    pub url: String,
    pub candidate: CandidateData,
}

#[derive(Hash, Debug)]
pub struct ApplicationData {
    pub id: i64,
    pub attachments: Vec<Attachment>,
    // pub current_stage: &'a applications::CurrentStage,
    pub current_stage_id: i64,
    pub candidate_id: i64,
    pub candidate: CandidateData,
}

impl ApplicationData {
    pub fn new(
        id: i64,
        attachments: Vec<Attachment>,
        current_stage_id: i64,
        candidate_id: i64,
        candidate: CandidateData,
    ) -> ApplicationData {
        ApplicationData {
            id,
            attachments,
            current_stage_id,
            candidate_id,
            candidate,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub candidate_id: i64,
    pub jobs: Vec<CurrentStage>,
    job_post_id: Option<i64>,
    pub status: String,
    pub current_stage: Option<CurrentStage>,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Clone)]
pub struct Attachment {
    pub filename: String,
    pub url: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
    //created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct CurrentStage {
    pub id: i64,
    name: String,
}