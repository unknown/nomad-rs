use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MigrateStrategy {
    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<String>,
    #[serde(rename = "HealthyDeadline", skip_serializing_if = "Option::is_none")]
    pub healthy_deadline: Option<i64>,
    #[serde(rename = "MaxParallel", skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<i32>,
    #[serde(rename = "MinHealthyTime", skip_serializing_if = "Option::is_none")]
    pub min_healthy_time: Option<i64>,
}
