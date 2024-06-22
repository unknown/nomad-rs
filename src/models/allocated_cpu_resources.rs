use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocatedCpuResources {
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
}
