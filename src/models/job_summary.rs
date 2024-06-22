use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JobSummary {
    #[serde(rename = "Children", skip_serializing_if = "Option::is_none")]
    pub children: Option<crate::models::JobChildrenSummary>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<::std::collections::HashMap<String, crate::models::TaskGroupSummary>>,
}
