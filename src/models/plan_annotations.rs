use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlanAnnotations {
    #[serde(rename = "DesiredTGUpdates", skip_serializing_if = "Option::is_none")]
    pub desired_tg_updates: Option<::std::collections::HashMap<String, crate::models::DesiredUpdates>>,
    #[serde(rename = "PreemptedAllocs", skip_serializing_if = "Option::is_none")]
    pub preempted_allocs: Option<Vec<crate::models::AllocationListStub>>,
}
