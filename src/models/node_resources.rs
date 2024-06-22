use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeResources {
    #[serde(rename = "Cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::models::NodeCpuResources>,
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::NodeDeviceResource>>,
    #[serde(rename = "Disk", skip_serializing_if = "Option::is_none")]
    pub disk: Option<crate::models::NodeDiskResources>,
    #[serde(rename = "MaxDynamicPort", skip_serializing_if = "Option::is_none")]
    pub max_dynamic_port: Option<i32>,
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::models::NodeMemoryResources>,
    #[serde(rename = "MinDynamicPort", skip_serializing_if = "Option::is_none")]
    pub min_dynamic_port: Option<i32>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
}
