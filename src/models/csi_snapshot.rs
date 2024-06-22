use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiSnapshot {
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "ExternalSourceVolumeID", skip_serializing_if = "Option::is_none")]
    pub external_source_volume_id: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsReady", skip_serializing_if = "Option::is_none")]
    pub is_ready: Option<bool>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "PluginID", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(rename = "Secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "SizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "SourceVolumeID", skip_serializing_if = "Option::is_none")]
    pub source_volume_id: Option<String>,
}
