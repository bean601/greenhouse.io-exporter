use serde::Deserialize;
use serde::Serialize;

pub type JobStage = Vec<JobStageElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobStageElement {
    id: i64,
    name: String,
    created_at: String,
    updated_at: String,
    job_id: i64,
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