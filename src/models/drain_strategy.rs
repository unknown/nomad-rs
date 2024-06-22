use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DrainStrategy {
    #[serde(rename = "Deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<i64>,
    #[serde(rename = "ForceDeadline", skip_serializing_if = "Option::is_none")]
    pub force_deadline: Option<String>,
    #[serde(rename = "IgnoreSystemJobs", skip_serializing_if = "Option::is_none")]
    pub ignore_system_jobs: Option<bool>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
}

