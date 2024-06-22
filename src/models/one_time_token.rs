use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OneTimeToken {
    #[serde(rename = "AccessorID", skip_serializing_if = "Option::is_none")]
    pub accessor_id: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ExpiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "OneTimeSecretID", skip_serializing_if = "Option::is_none")]
    pub one_time_secret_id: Option<String>,
}


