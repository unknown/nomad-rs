use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScalingEvent {
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<bool>,
    #[serde(rename = "EvalID", skip_serializing_if = "Option::is_none")]
    pub eval_id: Option<String>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "PreviousCount", skip_serializing_if = "Option::is_none")]
    pub previous_count: Option<i64>,
    #[serde(rename = "Time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
}
