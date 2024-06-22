use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HostVolumeInfo {
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
