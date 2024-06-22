use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JobAcl {
    #[serde(rename = "Group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Task", skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

