use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    #[serde(rename = "CPU", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i32>,
    #[serde(rename = "Cores", skip_serializing_if = "Option::is_none")]
    pub cores: Option<i32>,
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::RequestedDevice>>,
    #[serde(rename = "DiskMB", skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<i32>,
    #[serde(rename = "IOPS", skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "MemoryMB", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[serde(rename = "MemoryMaxMB", skip_serializing_if = "Option::is_none")]
    pub memory_max_mb: Option<i32>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
}
