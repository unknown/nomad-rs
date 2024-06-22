use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    #[serde(rename = "Destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "PropagationMode", skip_serializing_if = "Option::is_none")]
    pub propagation_mode: Option<String>,
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
}


