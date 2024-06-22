use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DriverInfo {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Detected", skip_serializing_if = "Option::is_none")]
    pub detected: Option<bool>,
    #[serde(rename = "HealthDescription", skip_serializing_if = "Option::is_none")]
    pub health_description: Option<String>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "UpdateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}
