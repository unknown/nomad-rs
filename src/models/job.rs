use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "Affinities", skip_serializing_if = "Option::is_none")]
    pub affinities: Option<Vec<crate::models::Affinity>>,
    #[serde(rename = "AllAtOnce", skip_serializing_if = "Option::is_none")]
    pub all_at_once: Option<bool>,
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<crate::models::Constraint>>,
    #[serde(rename = "ConsulNamespace", skip_serializing_if = "Option::is_none")]
    pub consul_namespace: Option<String>,
    #[serde(rename = "ConsulToken", skip_serializing_if = "Option::is_none")]
    pub consul_token: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<String>>,
    #[serde(rename = "DispatchIdempotencyToken", skip_serializing_if = "Option::is_none")]
    pub dispatch_idempotency_token: Option<String>,
    #[serde(rename = "Dispatched", skip_serializing_if = "Option::is_none")]
    pub dispatched: Option<bool>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<crate::models::MigrateStrategy>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Multiregion", skip_serializing_if = "Option::is_none")]
    pub multiregion: Option<crate::models::Multiregion>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NomadTokenID", skip_serializing_if = "Option::is_none")]
    pub nomad_token_id: Option<String>,
    #[serde(rename = "ParameterizedJob", skip_serializing_if = "Option::is_none")]
    pub parameterized_job: Option<crate::models::ParameterizedJobConfig>,
    #[serde(rename = "ParentID", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "Payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "Periodic", skip_serializing_if = "Option::is_none")]
    pub periodic: Option<crate::models::PeriodicConfig>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Reschedule", skip_serializing_if = "Option::is_none")]
    pub reschedule: Option<crate::models::ReschedulePolicy>,
    #[serde(rename = "Spreads", skip_serializing_if = "Option::is_none")]
    pub spreads: Option<Vec<crate::models::Spread>>,
    #[serde(rename = "Stable", skip_serializing_if = "Option::is_none")]
    pub stable: Option<bool>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<bool>,
    #[serde(rename = "SubmitTime", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<i64>,
    #[serde(rename = "TaskGroups", skip_serializing_if = "Option::is_none")]
    pub task_groups: Option<Vec<crate::models::TaskGroup>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Update", skip_serializing_if = "Option::is_none")]
    pub update: Option<crate::models::UpdateStrategy>,
    #[serde(rename = "VaultNamespace", skip_serializing_if = "Option::is_none")]
    pub vault_namespace: Option<String>,
    #[serde(rename = "VaultToken", skip_serializing_if = "Option::is_none")]
    pub vault_token: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}
