use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PeriodicConfig {
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ProhibitOverlap", skip_serializing_if = "Option::is_none")]
    pub prohibit_overlap: Option<bool>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(rename = "SpecType", skip_serializing_if = "Option::is_none")]
    pub spec_type: Option<String>,
    #[serde(rename = "TimeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}


