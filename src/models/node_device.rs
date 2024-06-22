use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeDevice {
    #[serde(rename = "HealthDescription", skip_serializing_if = "Option::is_none")]
    pub health_description: Option<String>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Locality", skip_serializing_if = "Option::is_none")]
    pub locality: Option<crate::models::NodeDeviceLocality>,
}
