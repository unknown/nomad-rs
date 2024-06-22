use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeMemoryResources {
    #[serde(rename = "MemoryMB", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i64>,
}
