use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsMultiregion", skip_serializing_if = "Option::is_none")]
    pub is_multiregion: Option<bool>,
    #[serde(rename = "JobCreateIndex", skip_serializing_if = "Option::is_none")]
    pub job_create_index: Option<i32>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "JobSpecModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_spec_modify_index: Option<i32>,
    #[serde(rename = "JobVersion", skip_serializing_if = "Option::is_none")]
    pub job_version: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "TaskGroups", skip_serializing_if = "Option::is_none")]
    pub task_groups: Option<::std::collections::HashMap<String, crate::models::DeploymentState>>,
}
