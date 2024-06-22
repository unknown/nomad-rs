use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "AddressMode", skip_serializing_if = "Option::is_none")]
    pub address_mode: Option<String>,
    #[serde(rename = "CanaryMeta", skip_serializing_if = "Option::is_none")]
    pub canary_meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "CanaryTags", skip_serializing_if = "Option::is_none")]
    pub canary_tags: Option<Vec<String>>,
    #[serde(rename = "CheckRestart", skip_serializing_if = "Option::is_none")]
    pub check_restart: Option<crate::models::CheckRestart>,
    #[serde(rename = "Checks", skip_serializing_if = "Option::is_none")]
    pub checks: Option<Vec<crate::models::ServiceCheck>>,
    #[serde(rename = "Connect", skip_serializing_if = "Option::is_none")]
    pub connect: Option<crate::models::ConsulConnect>,
    #[serde(rename = "EnableTagOverride", skip_serializing_if = "Option::is_none")]
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OnUpdate", skip_serializing_if = "Option::is_none")]
    pub on_update: Option<String>,
    #[serde(rename = "PortLabel", skip_serializing_if = "Option::is_none")]
    pub port_label: Option<String>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "TaggedAddresses", skip_serializing_if = "Option::is_none")]
    pub tagged_addresses: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "TaskName", skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
}
