use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeScoreMeta {
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NormScore", skip_serializing_if = "Option::is_none")]
    pub norm_score: Option<f64>,
    #[serde(rename = "Scores", skip_serializing_if = "Option::is_none")]
    pub scores: Option<::std::collections::HashMap<String, f64>>,
}
