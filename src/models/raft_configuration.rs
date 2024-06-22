use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RaftConfiguration {
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "Servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<crate::models::RaftServer>>,
}
