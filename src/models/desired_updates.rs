use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DesiredUpdates {
    #[serde(rename = "Canary", skip_serializing_if = "Option::is_none")]
    pub canary: Option<i32>,
    #[serde(rename = "DestructiveUpdate", skip_serializing_if = "Option::is_none")]
    pub destructive_update: Option<i32>,
    #[serde(rename = "Ignore", skip_serializing_if = "Option::is_none")]
    pub ignore: Option<i32>,
    #[serde(rename = "InPlaceUpdate", skip_serializing_if = "Option::is_none")]
    pub in_place_update: Option<i32>,
    #[serde(rename = "Migrate", skip_serializing_if = "Option::is_none")]
    pub migrate: Option<i32>,
    #[serde(rename = "Place", skip_serializing_if = "Option::is_none")]
    pub place: Option<i32>,
    #[serde(rename = "Preemptions", skip_serializing_if = "Option::is_none")]
    pub preemptions: Option<i32>,
    #[serde(rename = "Stop", skip_serializing_if = "Option::is_none")]
    pub stop: Option<i32>,
}


