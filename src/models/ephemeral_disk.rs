use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EphemeralDisk {
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<bool>,
    #[serde(rename = "SizeMB", skip_serializing_if = "Option::is_none")]
    pub size_mb: Option<i32>,
    #[serde(rename = "Sticky", skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,
}
