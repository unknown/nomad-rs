use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskHandle {
    #[serde(rename = "DriverState", skip_serializing_if = "Option::is_none")]
    pub driver_state: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}


