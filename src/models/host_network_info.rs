use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostNetworkInfo {
    #[serde(rename = "CIDR", skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(rename = "Interface", skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReservedPorts", skip_serializing_if = "Option::is_none")]
    pub reserved_ports: Option<String>,
}

