use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskGroupSummary {
    #[serde(rename = "Complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<i32>,
    #[serde(rename = "Failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(rename = "Lost", skip_serializing_if = "Option::is_none")]
    pub lost: Option<i32>,
    #[serde(rename = "Queued", skip_serializing_if = "Option::is_none")]
    pub queued: Option<i32>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    #[serde(rename = "Starting", skip_serializing_if = "Option::is_none")]
    pub starting: Option<i32>,
    #[serde(rename = "Unknown", skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
}


