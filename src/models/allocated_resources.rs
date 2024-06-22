use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AllocatedResources {
    #[serde(rename = "Shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<crate::models::AllocatedSharedResources>,
    #[serde(rename = "Tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<::std::collections::HashMap<String, crate::models::AllocatedTaskResources>>,
}


