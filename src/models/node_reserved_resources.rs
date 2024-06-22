use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeReservedResources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::models::NodeReservedCpuResources>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<crate::models::NodeReservedDiskResources>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::models::NodeReservedMemoryResources>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<crate::models::NodeReservedNetworkResources>,
}

