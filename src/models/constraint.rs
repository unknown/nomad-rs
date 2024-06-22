use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    #[serde(rename = "LTarget", skip_serializing_if = "Option::is_none")]
    pub l_target: Option<String>,
    #[serde(rename = "Operand", skip_serializing_if = "Option::is_none")]
    pub operand: Option<String>,
    #[serde(rename = "RTarget", skip_serializing_if = "Option::is_none")]
    pub r_target: Option<String>,
}

