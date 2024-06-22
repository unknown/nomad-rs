use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Multiregion {
    #[serde(rename = "Regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<crate::models::MultiregionRegion>>,
    #[serde(rename = "Strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<crate::models::MultiregionStrategy>,
}
