use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CheckRestart {
    #[serde(rename = "Grace", skip_serializing_if = "Option::is_none")]
    pub grace: Option<i64>,
    #[serde(rename = "IgnoreWarnings", skip_serializing_if = "Option::is_none")]
    pub ignore_warnings: Option<bool>,
    #[serde(rename = "Limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}


