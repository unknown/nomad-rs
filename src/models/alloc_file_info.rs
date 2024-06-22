use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AllocFileInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "IsDir", skip_serializing_if = "Option::is_none")]
    pub is_dir: Option<bool>,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "FileMode", skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<String>,
    #[serde(rename = "ModTime", skip_serializing_if = "Option::is_none")]
    pub mod_time: Option<String>,
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}