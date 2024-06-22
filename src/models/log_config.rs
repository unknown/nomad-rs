use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "MaxFileSizeMB", skip_serializing_if = "Option::is_none")]
    pub max_file_size_mb: Option<i32>,
    #[serde(rename = "MaxFiles", skip_serializing_if = "Option::is_none")]
    pub max_files: Option<i32>,
}
