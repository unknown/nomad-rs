use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulGatewayTlsConfig {
    #[serde(rename = "CipherSuites", skip_serializing_if = "Option::is_none")]
    pub cipher_suites: Option<Vec<String>>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "TLSMaxVersion", skip_serializing_if = "Option::is_none")]
    pub tls_max_version: Option<String>,
    #[serde(rename = "TLSMinVersion", skip_serializing_if = "Option::is_none")]
    pub tls_min_version: Option<String>,
}