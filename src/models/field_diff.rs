use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldDiff {
    #[serde(rename = "Annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<String>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "New", skip_serializing_if = "Option::is_none")]
    pub new: Option<String>,
    #[serde(rename = "Old", skip_serializing_if = "Option::is_none")]
    pub old: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
