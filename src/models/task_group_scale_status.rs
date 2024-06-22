use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskGroupScaleStatus {
    #[serde(rename = "Desired", skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::ScalingEvent>>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<i32>,
    #[serde(rename = "Placed", skip_serializing_if = "Option::is_none")]
    pub placed: Option<i32>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    #[serde(rename = "Unhealthy", skip_serializing_if = "Option::is_none")]
    pub unhealthy: Option<i32>,
}



