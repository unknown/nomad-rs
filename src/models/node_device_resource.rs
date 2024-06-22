use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeDeviceResource {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, crate::models::Attribute>>,
    #[serde(rename = "Instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::models::NodeDevice>>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}
