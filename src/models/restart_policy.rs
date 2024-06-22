use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RestartPolicy {
    #[serde(rename = "Attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}
