use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChangeScript {
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "FailOnError", skip_serializing_if = "Option::is_none")]
    pub fail_on_error: Option<bool>,
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}


