use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Evaluation {
    #[serde(rename = "AnnotatePlan", skip_serializing_if = "Option::is_none")]
    pub annotate_plan: Option<bool>,
    #[serde(rename = "BlockedEval", skip_serializing_if = "Option::is_none")]
    pub blocked_eval: Option<String>,
    #[serde(rename = "ClassEligibility", skip_serializing_if = "Option::is_none")]
    pub class_eligibility: Option<::std::collections::HashMap<String, bool>>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "CreateTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "DeploymentID", skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "EscapedComputedClass", skip_serializing_if = "Option::is_none")]
    pub escaped_computed_class: Option<bool>,
    #[serde(rename = "FailedTGAllocs", skip_serializing_if = "Option::is_none")]
    pub failed_tg_allocs: Option<::std::collections::HashMap<String, crate::models::AllocationMetric>>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobID", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobModifyIndex", skip_serializing_if = "Option::is_none")]
    pub job_modify_index: Option<i32>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "ModifyTime", skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextEval", skip_serializing_if = "Option::is_none")]
    pub next_eval: Option<String>,
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeModifyIndex", skip_serializing_if = "Option::is_none")]
    pub node_modify_index: Option<i32>,
    #[serde(rename = "PreviousEval", skip_serializing_if = "Option::is_none")]
    pub previous_eval: Option<String>,
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "QueuedAllocations", skip_serializing_if = "Option::is_none")]
    pub queued_allocations: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "QuotaLimitReached", skip_serializing_if = "Option::is_none")]
    pub quota_limit_reached: Option<String>,
    #[serde(rename = "RelatedEvals", skip_serializing_if = "Option::is_none")]
    pub related_evals: Option<Vec<crate::models::EvaluationStub>>,
    #[serde(rename = "SnapshotIndex", skip_serializing_if = "Option::is_none")]
    pub snapshot_index: Option<i32>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "TriggeredBy", skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Wait", skip_serializing_if = "Option::is_none")]
    pub wait: Option<i64>,
    #[serde(rename = "WaitUntil", skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<String>,
}
