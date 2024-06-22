use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiVolumeListStub {
    #[serde(rename = "AccessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    #[serde(rename = "AttachmentMode", skip_serializing_if = "Option::is_none")]
    pub attachment_mode: Option<String>,
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "ControllersExpected", skip_serializing_if = "Option::is_none")]
    pub controllers_expected: Option<i32>,
    #[serde(rename = "ControllersHealthy", skip_serializing_if = "Option::is_none")]
    pub controllers_healthy: Option<i32>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "CurrentReaders", skip_serializing_if = "Option::is_none")]
    pub current_readers: Option<i32>,
    #[serde(rename = "CurrentWriters", skip_serializing_if = "Option::is_none")]
    pub current_writers: Option<i32>,
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NodesExpected", skip_serializing_if = "Option::is_none")]
    pub nodes_expected: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "PluginID", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "ResourceExhausted", skip_serializing_if = "Option::is_none")]
    pub resource_exhausted: Option<String>,
    #[serde(rename = "Schedulable", skip_serializing_if = "Option::is_none")]
    pub schedulable: Option<bool>,
    #[serde(rename = "Topologies", skip_serializing_if = "Option::is_none")]
    pub topologies: Option<Vec<crate::models::CsiTopology>>,
}
