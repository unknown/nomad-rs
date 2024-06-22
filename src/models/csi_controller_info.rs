use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiControllerInfo {
    #[serde(rename = "SupportsAttachDetach", skip_serializing_if = "Option::is_none")]
    pub supports_attach_detach: Option<bool>,
    #[serde(rename = "SupportsClone", skip_serializing_if = "Option::is_none")]
    pub supports_clone: Option<bool>,
    #[serde(rename = "SupportsCondition", skip_serializing_if = "Option::is_none")]
    pub supports_condition: Option<bool>,
    #[serde(rename = "SupportsCreateDelete", skip_serializing_if = "Option::is_none")]
    pub supports_create_delete: Option<bool>,
    #[serde(rename = "SupportsCreateDeleteSnapshot", skip_serializing_if = "Option::is_none")]
    pub supports_create_delete_snapshot: Option<bool>,
    #[serde(rename = "SupportsExpand", skip_serializing_if = "Option::is_none")]
    pub supports_expand: Option<bool>,
    #[serde(rename = "SupportsGet", skip_serializing_if = "Option::is_none")]
    pub supports_get: Option<bool>,
    #[serde(rename = "SupportsGetCapacity", skip_serializing_if = "Option::is_none")]
    pub supports_get_capacity: Option<bool>,
    #[serde(rename = "SupportsListSnapshots", skip_serializing_if = "Option::is_none")]
    pub supports_list_snapshots: Option<bool>,
    #[serde(rename = "SupportsListVolumes", skip_serializing_if = "Option::is_none")]
    pub supports_list_volumes: Option<bool>,
    #[serde(rename = "SupportsListVolumesAttachedNodes", skip_serializing_if = "Option::is_none")]
    pub supports_list_volumes_attached_nodes: Option<bool>,
    #[serde(rename = "SupportsReadOnlyAttach", skip_serializing_if = "Option::is_none")]
    pub supports_read_only_attach: Option<bool>,
}
