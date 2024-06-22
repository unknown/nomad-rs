use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulSidecarService {
    #[serde(rename = "DisableDefaultTCPCheck", skip_serializing_if = "Option::is_none")]
    pub disable_default_tcp_check: Option<bool>,
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "Proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<crate::models::ConsulProxy>,
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}
