use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct JobChildrenSummary {
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<i64>,
    #[serde(rename = "Pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i64>,
}
