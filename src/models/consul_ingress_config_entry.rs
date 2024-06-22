use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulIngressConfigEntry {
    #[serde(rename = "Listeners", skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<crate::models::ConsulIngressListener>>,
    #[serde(rename = "TLS", skip_serializing_if = "Option::is_none")]
    pub tls: Option<crate::models::ConsulGatewayTlsConfig>,
}



