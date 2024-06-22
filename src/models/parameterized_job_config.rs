use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ParameterizedJobConfig {
    #[serde(rename = "MetaOptional", skip_serializing_if = "Option::is_none")]
    pub meta_optional: Option<Vec<String>>,
    #[serde(rename = "MetaRequired", skip_serializing_if = "Option::is_none")]
    pub meta_required: Option<Vec<String>>,
    #[serde(rename = "Payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}



