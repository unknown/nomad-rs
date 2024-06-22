use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulExposePath {
    #[serde(rename = "ListenerPort", skip_serializing_if = "Option::is_none")]
    pub listener_port: Option<String>,
    #[serde(rename = "LocalPathPort", skip_serializing_if = "Option::is_none")]
    pub local_path_port: Option<i32>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

