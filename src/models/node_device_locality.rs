use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeDeviceLocality {
    #[serde(rename = "PciBusID", skip_serializing_if = "Option::is_none")]
    pub pci_bus_id: Option<String>,
}
