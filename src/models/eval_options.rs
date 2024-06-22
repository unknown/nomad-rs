use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EvalOptions {
    #[serde(rename = "ForceReschedule", skip_serializing_if = "Option::is_none")]
    pub force_reschedule: Option<bool>,
}
