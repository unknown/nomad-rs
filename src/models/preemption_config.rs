use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PreemptionConfig {
    #[serde(rename = "BatchSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub batch_scheduler_enabled: Option<bool>,
    #[serde(rename = "ServiceSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub service_scheduler_enabled: Option<bool>,
    #[serde(rename = "SysBatchSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub sys_batch_scheduler_enabled: Option<bool>,
    #[serde(rename = "SystemSchedulerEnabled", skip_serializing_if = "Option::is_none")]
    pub system_scheduler_enabled: Option<bool>,
}

