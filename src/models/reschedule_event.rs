use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RescheduleEvent {
    #[serde(rename = "PrevAllocID", skip_serializing_if = "Option::is_none")]
    pub prev_alloc_id: Option<String>,
    #[serde(rename = "PrevNodeID", skip_serializing_if = "Option::is_none")]
    pub prev_node_id: Option<String>,
    #[serde(rename = "RescheduleTime", skip_serializing_if = "Option::is_none")]
    pub reschedule_time: Option<i64>,
}
