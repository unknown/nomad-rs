use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiPlugin {
    #[serde(rename = "Allocations", skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<crate::models::AllocationListStub>>,
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "Controllers", skip_serializing_if = "Option::is_none")]
    pub controllers: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
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
    #[serde(rename = "Nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "NodesExpected", skip_serializing_if = "Option::is_none")]
    pub nodes_expected: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
