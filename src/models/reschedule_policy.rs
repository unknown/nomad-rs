use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReschedulePolicy {
    #[serde(rename = "Attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(rename = "DelayFunction", skip_serializing_if = "Option::is_none")]
    pub delay_function: Option<String>,
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "MaxDelay", skip_serializing_if = "Option::is_none")]
    pub max_delay: Option<i64>,
    #[serde(rename = "Unlimited", skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
}


