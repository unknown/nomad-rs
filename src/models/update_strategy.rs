use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateStrategy {
    #[serde(rename = "AutoPromote", skip_serializing_if = "Option::is_none")]
    pub auto_promote: Option<bool>,
    #[serde(rename = "AutoRevert", skip_serializing_if = "Option::is_none")]
    pub auto_revert: Option<bool>,
    #[serde(rename = "Canary", skip_serializing_if = "Option::is_none")]
    pub canary: Option<i32>,
    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "HealthyDeadline", skip_serializing_if = "Option::is_none")]
    pub healthy_deadline: Option<i64>,
    #[serde(rename = "MaxParallel", skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<i32>,
    #[serde(rename = "MinHealthyTime", skip_serializing_if = "Option::is_none")]
    pub min_healthy_time: Option<i64>,
    #[serde(rename = "ProgressDeadline", skip_serializing_if = "Option::is_none")]
    pub progress_deadline: Option<i64>,
    #[serde(rename = "Stagger", skip_serializing_if = "Option::is_none")]
    pub stagger: Option<i64>,
}


