use serde::Deserialize;
use serde::Serialize;

pub type Candidates = Vec<Candidate>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    id: i64,
    first_name: String,
    last_name: String,
    company: String,
    title: String,
    created_at: String,
    updated_at: String,
    last_activity: String,
    is_private: bool,
    photo_url: Option<serde_json::Value>,
    attachments: Vec<Attachment>,
    application_ids: Vec<i64>,
    phone_numbers: Vec<Address>,
    addresses: Vec<Address>,
    email_addresses: Vec<Address>,
    website_addresses: Vec<Address>,
    social_media_addresses: Vec<Option<serde_json::Value>>,
    recruiter: Recruiter,
    coordinator: Option<serde_json::Value>,
    can_email: bool,
    tags: Vec<String>,
    applications: Vec<Application>,
    educations: Vec<Education>,
    employments: Vec<Employment>,
    linked_user_ids: Vec<i64>,
    custom_fields: CustomFields,
    keyed_custom_fields: KeyedCustomFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    value: String,
    #[serde(rename = "type")]
    address_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    id: i64,
    candidate_id: i64,
    prospect: bool,
    applied_at: String,
    rejected_at: Option<serde_json::Value>,
    last_activity_at: String,
    location: Option<Location>,
    source: Source,
    credited_to: Recruiter,
    rejection_reason: Option<serde_json::Value>,
    rejection_details: Option<serde_json::Value>,
    jobs: Vec<CurrentStage>,
    job_post_id: Option<i64>,
    status: String,
    current_stage: Option<CurrentStage>,
    answers: Vec<Option<serde_json::Value>>,
    prospective_office: Option<serde_json::Value>,
    prospective_department: Option<serde_json::Value>,
    prospect_detail: ProspectDetail,
    attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    filename: String,
    url: String,
    #[serde(rename = "type")]
    attachment_type: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recruiter {
    id: i64,
    first_name: String,
    last_name: String,
    name: String,
    employee_id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentStage {
    id: i64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProspectDetail {
    prospect_pool: Option<CurrentStage>,
    prospect_stage: Option<CurrentStage>,
    prospect_owner: Option<CurrentStage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    id: i64,
    public_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    desired_salary: String,
    work_remotely: bool,
    graduation_year: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    id: i64,
    school_name: String,
    degree: String,
    discipline: String,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Employment {
    id: i64,
    company_name: String,
    title: String,
    start_date: String,
    end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyedCustomFields {
    desired_salary: DesiredSalary,
    work_remotely: WorkRemotely,
    graduation_year_1: DesiredSalary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DesiredSalary {
    name: String,
    #[serde(rename = "type")]
    desired_salary_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkRemotely {
    name: String,
    #[serde(rename = "type")]
    work_remotely_type: String,
    value: bool,
}
