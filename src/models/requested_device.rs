use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RequestedDevice {
    #[serde(rename = "Affinities", skip_serializing_if = "Option::is_none")]
    pub affinities: Option<Vec<crate::models::Affinity>>,
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<crate::models::Constraint>>,
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

