use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeploymentState {
    #[serde(rename = "AutoRevert", skip_serializing_if = "Option::is_none")]
    pub auto_revert: Option<bool>,
    #[serde(rename = "DesiredCanaries", skip_serializing_if = "Option::is_none")]
    pub desired_canaries: Option<i32>,
    #[serde(rename = "DesiredTotal", skip_serializing_if = "Option::is_none")]
    pub desired_total: Option<i32>,
    #[serde(rename = "HealthyAllocs", skip_serializing_if = "Option::is_none")]
    pub healthy_allocs: Option<i32>,
    #[serde(rename = "PlacedAllocs", skip_serializing_if = "Option::is_none")]
    pub placed_allocs: Option<i32>,
    #[serde(rename = "PlacedCanaries", skip_serializing_if = "Option::is_none")]
    pub placed_canaries: Option<Vec<String>>,
    #[serde(rename = "ProgressDeadline", skip_serializing_if = "Option::is_none")]
    pub progress_deadline: Option<i64>,
    #[serde(rename = "Promoted", skip_serializing_if = "Option::is_none")]
    pub promoted: Option<bool>,
    #[serde(rename = "RequireProgressBy", skip_serializing_if = "Option::is_none")]
    pub require_progress_by: Option<String>,
    #[serde(rename = "UnhealthyAllocs", skip_serializing_if = "Option::is_none")]
    pub unhealthy_allocs: Option<i32>,
}
