use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeCpuResources {
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
    #[serde(rename = "ReservableCpuCores", skip_serializing_if = "Option::is_none")]
    pub reservable_cpu_cores: Option<Vec<i32>>,
    #[serde(rename = "TotalCpuCores", skip_serializing_if = "Option::is_none")]
    pub total_cpu_cores: Option<i32>,
}
