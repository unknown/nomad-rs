use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DesiredTransition {
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<bool>,
    #[serde(rename = "Reschedule", skip_serializing_if = "Option::is_none")]
    pub reschedule: Option<bool>,
}
