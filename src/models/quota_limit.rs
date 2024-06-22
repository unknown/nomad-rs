use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct QuotaLimit {
    #[serde(rename = "Hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RegionLimit", skip_serializing_if = "Option::is_none")]
    pub region_limit: Option<crate::models::Resources>,
    #[serde(rename = "VariablesLimit", skip_serializing_if = "Option::is_none")]
    pub variables_limit: Option<i32>,
}

