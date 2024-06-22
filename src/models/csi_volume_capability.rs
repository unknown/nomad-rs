use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiVolumeCapability {
    #[serde(rename = "AccessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    #[serde(rename = "AttachmentMode", skip_serializing_if = "Option::is_none")]
    pub attachment_mode: Option<String>,
}
