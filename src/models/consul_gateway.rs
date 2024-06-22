use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulGateway {
    #[serde(rename = "Ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<crate::models::ConsulIngressConfigEntry>,
    #[serde(rename = "Mesh", skip_serializing_if = "Option::is_none")]
    pub mesh: Option<serde_json::Value>,
    #[serde(rename = "Proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<crate::models::ConsulGatewayProxy>,
    #[serde(rename = "Terminating", skip_serializing_if = "Option::is_none")]
    pub terminating: Option<crate::models::ConsulTerminatingConfigEntry>,
}

