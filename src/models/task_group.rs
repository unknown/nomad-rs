use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TaskGroup {
    #[serde(rename = "Affinities", skip_serializing_if = "Option::is_none")]
    pub affinities: Option<Vec<crate::models::Affinity>>,
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<crate::models::Constraint>>,
    #[serde(rename = "Consul", skip_serializing_if = "Option::is_none")]
    pub consul: Option<crate::models::Consul>,
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "EphemeralDisk", skip_serializing_if = "Option::is_none")]
    pub ephemeral_disk: Option<crate::models::EphemeralDisk>,
    #[serde(rename = "MaxClientDisconnect", skip_serializing_if = "Option::is_none")]
    pub max_client_disconnect: Option<i64>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<crate::models::MigrateStrategy>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::NetworkResource>>,
    #[serde(rename = "ReschedulePolicy", skip_serializing_if = "Option::is_none")]
    pub reschedule_policy: Option<crate::models::ReschedulePolicy>,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<crate::models::RestartPolicy>,
    #[serde(rename = "Scaling", skip_serializing_if = "Option::is_none")]
    pub scaling: Option<crate::models::ScalingPolicy>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<crate::models::Service>>,
    #[serde(rename = "ShutdownDelay", skip_serializing_if = "Option::is_none")]
    pub shutdown_delay: Option<i64>,
    #[serde(rename = "Spreads", skip_serializing_if = "Option::is_none")]
    pub spreads: Option<Vec<crate::models::Spread>>,
    #[serde(rename = "StopAfterClientDisconnect", skip_serializing_if = "Option::is_none")]
    pub stop_after_client_disconnect: Option<i64>,
    #[serde(rename = "Tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<crate::models::Task>>,
    #[serde(rename = "Update", skip_serializing_if = "Option::is_none")]
    pub update: Option<crate::models::UpdateStrategy>,
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<::std::collections::HashMap<String, crate::models::VolumeRequest>>,
}


