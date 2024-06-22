use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeRequest {
    #[serde(rename = "AccessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    #[serde(rename = "AttachmentMode", skip_serializing_if = "Option::is_none")]
    pub attachment_mode: Option<String>,
    #[serde(rename = "MountOptions", skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<crate::models::CsiMountOptions>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PerAlloc", skip_serializing_if = "Option::is_none")]
    pub per_alloc: Option<bool>,
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
