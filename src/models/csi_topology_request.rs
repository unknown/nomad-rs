use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsiTopologyRequest {
    #[serde(rename = "Preferred", skip_serializing_if = "Option::is_none")]
    pub preferred: Option<Vec<crate::models::CsiTopology>>,
    #[serde(rename = "Required", skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<crate::models::CsiTopology>>,
}

