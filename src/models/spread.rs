use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Spread {
    #[serde(rename = "Attribute", skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "SpreadTarget", skip_serializing_if = "Option::is_none")]
    pub spread_target: Option<Vec<crate::models::SpreadTarget>>,
    #[serde(rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
