use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulProxy {
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "ExposeConfig", skip_serializing_if = "Option::is_none")]
    pub expose_config: Option<crate::models::ConsulExposeConfig>,
    #[serde(rename = "LocalServiceAddress", skip_serializing_if = "Option::is_none")]
    pub local_service_address: Option<String>,
    #[serde(rename = "LocalServicePort", skip_serializing_if = "Option::is_none")]
    pub local_service_port: Option<i32>,
    #[serde(rename = "Upstreams", skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<crate::models::ConsulUpstream>>,
}
