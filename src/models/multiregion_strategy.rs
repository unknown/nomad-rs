use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MultiregionStrategy {
    #[serde(rename = "MaxParallel", skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<i32>,
    #[serde(rename = "OnFailure", skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
}


