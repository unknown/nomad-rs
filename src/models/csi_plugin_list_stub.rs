use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiPluginListStub {
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "ControllersExpected", skip_serializing_if = "Option::is_none")]
    pub controllers_expected: Option<i32>,
    #[serde(rename = "ControllersHealthy", skip_serializing_if = "Option::is_none")]
    pub controllers_healthy: Option<i32>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "NodesExpected", skip_serializing_if = "Option::is_none")]
    pub nodes_expected: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
