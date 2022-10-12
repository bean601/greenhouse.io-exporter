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
pub struct ApplicationData<'a> {
    pub id: i64,
    pub attachments: &'a Vec<Attachment>,
    // pub current_stage: &'a applications::CurrentStage,
    pub current_stage_id: i64,
    pub candidate_id: i64,
    pub candidate: CandidateData,
}

impl ApplicationData<'_> {
    pub fn new<'a>(
        id: i64,
        attachments: &'a Vec<Attachment>,
        current_stage_id: i64,
        candidate_id: i64,
        candidate: CandidateData,
    ) -> ApplicationData<'a> {
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
    // prospect: bool,
    // applied_at: String,
    // rejected_at: Option<serde_json::Value>,
    // last_activity_at: String,
    // location: Option<ApplicationLocation>,
    // source: Source,
    // credited_to: CreditedTo,
    // rejection_reason: Option<serde_json::Value>,
    // rejection_details: Option<serde_json::Value>,
    pub jobs: Vec<CurrentStage>,
    job_post_id: Option<i64>,
    pub status: String,
    pub current_stage: Option<CurrentStage>,
    // answers: Vec<Answer>,
    // prospective_office: Option<ProspectiveOffice>,
    // prospective_department: Option<ProspectiveDepartment>,
    // prospect_detail: ProspectDetail,
    // custom_fields: CustomFields,
    // keyed_custom_fields: KeyedCustomFields,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    question: String,
    answer: String,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct Attachment {
    pub filename: String,
    pub url: String,
    #[serde(rename = "type")]
    pub attachment_type: String,
    //created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditedTo {
    id: i64,
    first_name: String,
    last_name: String,
    name: String,
    employee_id: String,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct CurrentStage {
    pub id: i64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    application_custom_test: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyedCustomFields {
    application_custom_test: ApplicationCustomTest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCustomTest {
    name: String,
    #[serde(rename = "type")]
    application_custom_test_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationLocation {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProspectDetail {
    prospect_pool: Option<CurrentStage>,
    prospect_stage: Option<CurrentStage>,
    prospect_owner: Option<CurrentStage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProspectiveDepartment {
    parent_id: Option<serde_json::Value>,
    name: String,
    id: i64,
    external_id: Option<serde_json::Value>,
    child_ids: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProspectiveOffice {
    primary_contact_user_id: Option<serde_json::Value>,
    parent_id: Option<serde_json::Value>,
    name: String,
    location: ProspectiveOfficeLocation,
    id: i64,
    external_id: Option<serde_json::Value>,
    child_ids: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProspectiveOfficeLocation {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    id: i64,
    public_name: String,
}
