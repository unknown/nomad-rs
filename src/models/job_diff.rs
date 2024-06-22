use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JobDiff {
    #[serde(rename = "Fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::FieldDiff>>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Objects", skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<crate::models::ObjectDiff>>,
    #[serde(rename = "TaskGroups", skip_serializing_if = "Option::is_none")]
    pub task_groups: Option<Vec<crate::models::TaskGroupDiff>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
