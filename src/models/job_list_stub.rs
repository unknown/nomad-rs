use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JobListStub {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<String>>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "JobSummary", skip_serializing_if = "Option::is_none")]
    pub job_summary: Option<crate::models::JobSummary>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "ParameterizedJob", skip_serializing_if = "Option::is_none")]
    pub parameterized_job: Option<bool>,
    #[serde(rename = "ParentID", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "Periodic", skip_serializing_if = "Option::is_none")]
    pub periodic: Option<bool>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    #[serde(rename = "SubmitTime", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<i64>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}


