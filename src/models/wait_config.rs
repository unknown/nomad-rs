use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WaitConfig {
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
}

