use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskLifecycle {
    #[serde(rename = "Hook", skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(rename = "Sidecar", skip_serializing_if = "Option::is_none")]
    pub sidecar: Option<bool>,
}


