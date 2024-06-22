use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocatedMemoryResources {
    #[serde(rename = "MemoryMB", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i64>,
    #[serde(rename = "MemoryMaxMB", skip_serializing_if = "Option::is_none")]
    pub memory_max_mb: Option<i64>,
}
