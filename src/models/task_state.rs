use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskState {
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::TaskEvent>>,
    #[serde(rename = "Failed", skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "FinishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "LastRestart", skip_serializing_if = "Option::is_none")]
    pub last_restart: Option<String>,
    #[serde(rename = "Restarts", skip_serializing_if = "Option::is_none")]
    pub restarts: Option<i32>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TaskHandle", skip_serializing_if = "Option::is_none")]
    pub task_handle: Option<crate::models::TaskHandle>,
}


