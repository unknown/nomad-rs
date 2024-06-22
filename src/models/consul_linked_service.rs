use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulLinkedService {
    #[serde(rename = "CAFile", skip_serializing_if = "Option::is_none")]
    pub ca_file: Option<String>,
    #[serde(rename = "CertFile", skip_serializing_if = "Option::is_none")]
    pub cert_file: Option<String>,
    #[serde(rename = "KeyFile", skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SNI", skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
}


