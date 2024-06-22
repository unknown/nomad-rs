use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeListStub {
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "Drain", skip_serializing_if = "Option::is_none")]
    pub drain: Option<bool>,
    #[serde(rename = "Drivers", skip_serializing_if = "Option::is_none")]
    pub drivers: Option<::std::collections::HashMap<String, crate::models::DriverInfo>>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastDrain", skip_serializing_if = "Option::is_none")]
    pub last_drain: Option<crate::models::DrainMetadata>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeClass", skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    #[serde(rename = "NodeResources", skip_serializing_if = "Option::is_none")]
    pub node_resources: Option<crate::models::NodeResources>,
    #[serde(rename = "ReservedResources", skip_serializing_if = "Option::is_none")]
    pub reserved_resources: Option<crate::models::NodeReservedResources>,
    #[serde(rename = "SchedulingEligibility", skip_serializing_if = "Option::is_none")]
    pub scheduling_eligibility: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
