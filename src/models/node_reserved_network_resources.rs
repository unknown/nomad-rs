use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeReservedNetworkResources {
    #[serde(rename = "ReservedHostPorts", skip_serializing_if = "Option::is_none")]
    pub reserved_host_ports: Option<String>,
}
