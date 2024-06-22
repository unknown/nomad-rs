use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulExposeConfig {
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<crate::models::ConsulExposePath>>,
}


