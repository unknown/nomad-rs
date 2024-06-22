use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulIngressListener {
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::ConsulIngressService>>,
}

