use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Allocation {
    #[serde(rename = "AllocModifyIndex", skip_serializing_if = "Option::is_none")]
    pub alloc_modify_index: Option<i32>,
    #[serde(rename = "AllocatedResources", skip_serializing_if = "Option::is_none")]
    pub allocated_resources: Option<crate::models::AllocatedResources>,
    #[serde(rename = "ClientDescription", skip_serializing_if = "Option::is_none")]
    pub client_description: Option<String>,
    #[serde(rename = "ClientStatus", skip_serializing_if = "Option::is_none")]
    pub client_status: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "DeploymentStatus", skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<crate::models::AllocDeploymentStatus>,
    #[serde(rename = "DesiredDescription", skip_serializing_if = "Option::is_none")]
    pub desired_description: Option<String>,
    #[serde(rename = "DesiredStatus", skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    #[serde(rename = "DesiredTransition", skip_serializing_if = "Option::is_none")]
    pub desired_transition: Option<crate::models::DesiredTransition>,
    #[serde(rename = "EvalID", skip_serializing_if = "Option::is_none")]
    pub eval_id: Option<String>,
    #[serde(rename = "FollowupEvalID", skip_serializing_if = "Option::is_none")]
    pub followup_eval_id: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Job", skip_serializing_if = "Option::is_none")]
    pub job: Option<crate::models::Job>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<crate::models::AllocationMetric>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "ModifyTime", skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextAllocation", skip_serializing_if = "Option::is_none")]
    pub next_allocation: Option<String>,
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeName", skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "PreemptedAllocations", skip_serializing_if = "Option::is_none")]
    pub preempted_allocations: Option<Vec<String>>,
    #[serde(rename = "PreemptedByAllocation", skip_serializing_if = "Option::is_none")]
    pub preempted_by_allocation: Option<String>,
    #[serde(rename = "PreviousAllocation", skip_serializing_if = "Option::is_none")]
    pub previous_allocation: Option<String>,
    #[serde(rename = "RescheduleTracker", skip_serializing_if = "Option::is_none")]
    pub reschedule_tracker: Option<crate::models::RescheduleTracker>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<crate::models::Resources>,
    #[serde(rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "TaskGroup", skip_serializing_if = "Option::is_none")]
    pub task_group: Option<String>,
    #[serde(rename = "TaskResources", skip_serializing_if = "Option::is_none")]
    pub task_resources: Option<::std::collections::HashMap<String, crate::models::Resources>>,
    #[serde(rename = "TaskStates", skip_serializing_if = "Option::is_none")]
    pub task_states: Option<::std::collections::HashMap<String, crate::models::TaskState>>,
}
