use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiMountOptions {
    #[serde(rename = "FSType", skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[serde(rename = "MountFlags", skip_serializing_if = "Option::is_none")]
    pub mount_flags: Option<Vec<String>>,
}
