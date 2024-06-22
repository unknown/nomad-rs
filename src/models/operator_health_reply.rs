use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OperatorHealthReply {
    #[serde(rename = "FailureTolerance", skip_serializing_if = "Option::is_none")]
    pub failure_tolerance: Option<i32>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "Servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<crate::models::ServerHealth>>,
}
