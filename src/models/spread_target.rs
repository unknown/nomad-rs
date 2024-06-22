use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpreadTarget {
    #[serde(rename = "Percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
