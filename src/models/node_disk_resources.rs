use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeDiskResources {
    #[serde(rename = "DiskMB", skip_serializing_if = "Option::is_none")]
    pub disk_mb: Option<i64>,
}
