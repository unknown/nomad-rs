use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskArtifact {
    #[serde(rename = "GetterHeaders", skip_serializing_if = "Option::is_none")]
    pub getter_headers: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "GetterMode", skip_serializing_if = "Option::is_none")]
    pub getter_mode: Option<String>,
    #[serde(rename = "GetterOptions", skip_serializing_if = "Option::is_none")]
    pub getter_options: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "GetterSource", skip_serializing_if = "Option::is_none")]
    pub getter_source: Option<String>,
    #[serde(rename = "RelativeDest", skip_serializing_if = "Option::is_none")]
    pub relative_dest: Option<String>,
}

