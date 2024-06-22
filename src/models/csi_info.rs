use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiInfo {
    #[serde(rename = "AllocID", skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<String>,
    #[serde(rename = "ControllerInfo", skip_serializing_if = "Option::is_none")]
    pub controller_info: Option<crate::models::CsiControllerInfo>,
    #[serde(rename = "HealthDescription", skip_serializing_if = "Option::is_none")]
    pub health_description: Option<String>,
    #[serde(rename = "Healthy", skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(rename = "NodeInfo", skip_serializing_if = "Option::is_none")]
    pub node_info: Option<crate::models::CsiNodeInfo>,
    #[serde(rename = "PluginID", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(rename = "RequiresControllerPlugin", skip_serializing_if = "Option::is_none")]
    pub requires_controller_plugin: Option<bool>,
    #[serde(rename = "RequiresTopologies", skip_serializing_if = "Option::is_none")]
    pub requires_topologies: Option<bool>,
    #[serde(rename = "UpdateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}
