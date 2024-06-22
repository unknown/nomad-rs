use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskEvent {
    #[serde(rename = "Details", skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "DiskLimit", skip_serializing_if = "Option::is_none")]
    pub disk_limit: Option<i64>,
    #[serde(rename = "DiskSize", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    #[serde(rename = "DisplayMessage", skip_serializing_if = "Option::is_none")]
    pub display_message: Option<String>,
    #[serde(rename = "DownloadError", skip_serializing_if = "Option::is_none")]
    pub download_error: Option<String>,
    #[serde(rename = "DriverError", skip_serializing_if = "Option::is_none")]
    pub driver_error: Option<String>,
    #[serde(rename = "DriverMessage", skip_serializing_if = "Option::is_none")]
    pub driver_message: Option<String>,
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "FailedSibling", skip_serializing_if = "Option::is_none")]
    pub failed_sibling: Option<String>,
    #[serde(rename = "FailsTask", skip_serializing_if = "Option::is_none")]
    pub fails_task: Option<bool>,
    #[serde(rename = "GenericSource", skip_serializing_if = "Option::is_none")]
    pub generic_source: Option<String>,
    #[serde(rename = "KillError", skip_serializing_if = "Option::is_none")]
    pub kill_error: Option<String>,
    #[serde(rename = "KillReason", skip_serializing_if = "Option::is_none")]
    pub kill_reason: Option<String>,
    #[serde(rename = "KillTimeout", skip_serializing_if = "Option::is_none")]
    pub kill_timeout: Option<i64>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RestartReason", skip_serializing_if = "Option::is_none")]
    pub restart_reason: Option<String>,
    #[serde(rename = "SetupError", skip_serializing_if = "Option::is_none")]
    pub setup_error: Option<String>,
    #[serde(rename = "Signal", skip_serializing_if = "Option::is_none")]
    pub signal: Option<i32>,
    #[serde(rename = "StartDelay", skip_serializing_if = "Option::is_none")]
    pub start_delay: Option<i64>,
    #[serde(rename = "TaskSignal", skip_serializing_if = "Option::is_none")]
    pub task_signal: Option<String>,
    #[serde(rename = "TaskSignalReason", skip_serializing_if = "Option::is_none")]
    pub task_signal_reason: Option<String>,
    #[serde(rename = "Time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "ValidationError", skip_serializing_if = "Option::is_none")]
    pub validation_error: Option<String>,
    #[serde(rename = "VaultError", skip_serializing_if = "Option::is_none")]
    pub vault_error: Option<String>,
}



