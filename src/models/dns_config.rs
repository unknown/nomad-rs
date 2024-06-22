use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DnsConfig {
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "Searches", skip_serializing_if = "Option::is_none")]
    pub searches: Option<Vec<String>>,
    #[serde(rename = "Servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,
}

