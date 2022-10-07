use serde::Deserialize;
use serde::Serialize;

pub type Jobs = Vec<Job>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: i64,
    pub name: String,
    pub requisition_id: String,
    // notes: String,
    // confidential: bool,
    // status: String,
    // created_at: String,
    // opened_at: String,
    // closed_at: String,
    // updated_at: String,
    // is_template: bool,
    // copied_from_id: i64,
    // departments: Vec<Department>,
    // offices: Vec<Office>,
    // custom_fields: CustomFields,
    // keyed_custom_fields: KeyedCustomFields,
    // hiring_team: HiringTeam,
    // openings: Vec<Opening>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    employment_type: String,
    maximum_budget: String,
    salary_range: ValueClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueClass {
    min_value: i64,
    max_value: i64,
    unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    id: i64,
    name: String,
    parent_id: i64,
    child_ids: Vec<i64>,
    external_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HiringTeam {
    hiring_managers: Vec<Coordinator>,
    recruiters: Vec<Coordinator>,
    coordinators: Vec<Coordinator>,
    sourcers: Vec<Coordinator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinator {
    id: i64,
    first_name: String,
    last_name: String,
    name: String,
    employee_id: String,
    responsible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyedCustomFields {
    employment_type: Budget,
    budget: Budget,
    salary_range: KeyedCustomFieldsSalaryRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    name: String,
    #[serde(rename = "type")]
    budget_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyedCustomFieldsSalaryRange {
    name: String,
    #[serde(rename = "type")]
    salary_range_type: String,
    value: ValueClass,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Office {
    id: i64,
    name: String,
    location: Location,
    primary_contact_user_id: i64,
    parent_id: i64,
    child_ids: Vec<i64>,
    external_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opening {
    id: i64,
    opening_id: Option<String>,
    status: String,
    opened_at: String,
    closed_at: Option<String>,
    application_id: Option<i64>,
    close_reason: Option<CloseReason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseReason {
    id: i64,
    name: String,
}
