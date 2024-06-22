use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[serde(rename = "HostNetwork", skip_serializing_if = "Option::is_none")]
    pub host_network: Option<String>,
    #[serde(rename = "Label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "To", skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

