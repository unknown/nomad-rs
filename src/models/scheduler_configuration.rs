use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SchedulerConfiguration {
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "MemoryOversubscriptionEnabled", skip_serializing_if = "Option::is_none")]
    pub memory_oversubscription_enabled: Option<bool>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "PauseEvalBroker", skip_serializing_if = "Option::is_none")]
    pub pause_eval_broker: Option<bool>,
    #[serde(rename = "PreemptionConfig", skip_serializing_if = "Option::is_none")]
    pub preemption_config: Option<crate::models::PreemptionConfig>,
    #[serde(rename = "RejectJobRegistration", skip_serializing_if = "Option::is_none")]
    pub reject_job_registration: Option<bool>,
    #[serde(rename = "SchedulerAlgorithm", skip_serializing_if = "Option::is_none")]
    pub scheduler_algorithm: Option<String>,
}

