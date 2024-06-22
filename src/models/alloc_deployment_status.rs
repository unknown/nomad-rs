use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocDeploymentStatus {
    #[serde(rename = "Canary", skip_serializing_if = "Option::is_none")]
    pub canary: Option<bool>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
