use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCapabilities {
    #[serde(rename = "DisabledTaskDrivers", skip_serializing_if = "Option::is_none")]
    pub disabled_task_drivers: Option<Vec<String>>,
    #[serde(rename = "EnabledTaskDrivers", skip_serializing_if = "Option::is_none")]
    pub enabled_task_drivers: Option<Vec<String>>,
}

