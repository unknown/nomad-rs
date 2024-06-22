use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NetworkResource {
    #[serde(rename = "CIDR", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "DNS", skip_serializing_if = "Option::is_none")]
    pub dns: Option<crate::models::DnsConfig>,
    #[serde(rename = "Device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    #[serde(rename = "DynamicPorts", skip_serializing_if = "Option::is_none")]
    pub dynamic_ports: Option<Vec<crate::models::Port>>,
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "MBits", skip_serializing_if = "Option::is_none")]
    pub m_bits: Option<i32>,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "ReservedPorts", skip_serializing_if = "Option::is_none")]
    pub reserved_ports: Option<Vec<crate::models::Port>>,
}
