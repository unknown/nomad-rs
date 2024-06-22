use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DrainSpec {
    #[serde(rename = "Deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<i64>,
    #[serde(rename = "IgnoreSystemJobs", skip_serializing_if = "Option::is_none")]
    pub ignore_system_jobs: Option<bool>,
}
