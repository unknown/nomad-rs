use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocationMetric {
    #[serde(rename = "AllocationTime", skip_serializing_if = "Option::is_none")]
    pub allocation_time: Option<i64>,
    #[serde(rename = "ClassExhausted", skip_serializing_if = "Option::is_none")]
    pub class_exhausted: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "ClassFiltered", skip_serializing_if = "Option::is_none")]
    pub class_filtered: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "CoalescedFailures", skip_serializing_if = "Option::is_none")]
    pub coalesced_failures: Option<i32>,
    #[serde(rename = "ConstraintFiltered", skip_serializing_if = "Option::is_none")]
    pub constraint_filtered: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "DimensionExhausted", skip_serializing_if = "Option::is_none")]
    pub dimension_exhausted: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "NodesAvailable", skip_serializing_if = "Option::is_none")]
    pub nodes_available: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "NodesEvaluated", skip_serializing_if = "Option::is_none")]
    pub nodes_evaluated: Option<i32>,
    #[serde(rename = "NodesExhausted", skip_serializing_if = "Option::is_none")]
    pub nodes_exhausted: Option<i32>,
    #[serde(rename = "NodesFiltered", skip_serializing_if = "Option::is_none")]
    pub nodes_filtered: Option<i32>,
    #[serde(rename = "QuotaExhausted", skip_serializing_if = "Option::is_none")]
    pub quota_exhausted: Option<Vec<String>>,
    #[serde(rename = "ResourcesExhausted", skip_serializing_if = "Option::is_none")]
    pub resources_exhausted: Option<::std::collections::HashMap<String, crate::models::Resources>>,
    #[serde(rename = "ScoreMetaData", skip_serializing_if = "Option::is_none")]
    pub score_meta_data: Option<Vec<crate::models::NodeScoreMeta>>,
    #[serde(rename = "Scores", skip_serializing_if = "Option::is_none")]
    pub scores: Option<::std::collections::HashMap<String, f64>>,
}


