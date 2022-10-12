use serde::Deserialize;
use serde::Serialize;

pub type JobStage = Vec<JobStageElement>;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct JobStageData<'a> {
    pub id: i64,
    pub name: &'a str,
}

impl JobStageData<'_> {
    pub fn new(id: i64, name: &str) -> JobStageData {
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
    // priority: i64,
    // interviews: Vec<Interview>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interview {
    id: i64,
    name: String,
    schedulable: bool,
    estimated_minutes: i64,
    default_interviewer_users: Vec<DefaultInterviewerUser>,
    interview_kit: InterviewKit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultInterviewerUser {
    id: i64,
    first_name: String,
    last_name: String,
    name: String,
    employee_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterviewKit {
    id: i64,
    content: String,
    questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    id: i64,
    question: String,
}
