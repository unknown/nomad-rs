use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskCsiPluginConfig {
    #[serde(rename = "HealthTimeout", skip_serializing_if = "Option::is_none")]
    pub health_timeout: Option<i64>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MountDir", skip_serializing_if = "Option::is_none")]
    pub mount_dir: Option<String>,
    #[serde(rename = "StagePublishBaseDir", skip_serializing_if = "Option::is_none")]
    pub stage_publish_base_dir: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}


