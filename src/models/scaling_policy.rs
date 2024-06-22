use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ScalingPolicy {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
