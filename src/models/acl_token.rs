use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AclToken {
    #[serde(rename = "AccessorID", skip_serializing_if = "Option::is_none")]
    pub accessor_id: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "ExpirationTTL", skip_serializing_if = "Option::is_none")]
    pub expiration_ttl: Option<i64>,
    #[serde(rename = "ExpirationTime", skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "Global", skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
    #[serde(rename = "Roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::AclTokenRoleLink>>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}