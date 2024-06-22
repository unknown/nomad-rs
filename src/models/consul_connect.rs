use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulConnect {
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<crate::models::ConsulGateway>,
    #[serde(rename = "Native", skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    #[serde(rename = "SidecarService", skip_serializing_if = "Option::is_none")]
    pub sidecar_service: Option<crate::models::ConsulSidecarService>,
    #[serde(rename = "SidecarTask", skip_serializing_if = "Option::is_none")]
    pub sidecar_task: Option<crate::models::SidecarTask>,
}
