use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiTopology {
    #[serde(rename = "Segments", skip_serializing_if = "Option::is_none")]
    pub segments: Option<::std::collections::HashMap<String, String>>,
}
