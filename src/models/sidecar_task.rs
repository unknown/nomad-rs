use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SidecarTask {
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "KillSignal", skip_serializing_if = "Option::is_none")]
    pub kill_signal: Option<String>,
    #[serde(rename = "KillTimeout", skip_serializing_if = "Option::is_none")]
    pub kill_timeout: Option<i64>,
    #[serde(rename = "LogConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<crate::models::LogConfig>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<crate::models::Resources>,
    #[serde(rename = "ShutdownDelay", skip_serializing_if = "Option::is_none")]
    pub shutdown_delay: Option<i64>,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
