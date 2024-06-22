use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DispatchPayloadConfig {
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
