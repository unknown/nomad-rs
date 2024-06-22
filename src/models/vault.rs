use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vault {
    #[serde(rename = "ChangeMode", skip_serializing_if = "Option::is_none")]
    pub change_mode: Option<String>,
    #[serde(rename = "ChangeSignal", skip_serializing_if = "Option::is_none")]
    pub change_signal: Option<String>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<bool>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}


