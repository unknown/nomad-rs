use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiNodeInfo {
    #[serde(rename = "AccessibleTopology", skip_serializing_if = "Option::is_none")]
    pub accessible_topology: Option<crate::models::CsiTopology>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MaxVolumes", skip_serializing_if = "Option::is_none")]
    pub max_volumes: Option<i64>,
    #[serde(rename = "RequiresNodeStageVolume", skip_serializing_if = "Option::is_none")]
    pub requires_node_stage_volume: Option<bool>,
    #[serde(rename = "SupportsCondition", skip_serializing_if = "Option::is_none")]
    pub supports_condition: Option<bool>,
    #[serde(rename = "SupportsExpand", skip_serializing_if = "Option::is_none")]
    pub supports_expand: Option<bool>,
    #[serde(rename = "SupportsStats", skip_serializing_if = "Option::is_none")]
    pub supports_stats: Option<bool>,
}
