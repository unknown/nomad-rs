use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(rename = "Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<bool>,
    #[serde(rename = "Float", skip_serializing_if = "Option::is_none")]
    pub float: Option<f64>,
    #[serde(rename = "Int", skip_serializing_if = "Option::is_none")]
    pub int: Option<i64>,
    #[serde(rename = "String", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
