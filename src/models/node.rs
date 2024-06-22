use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "CSIControllerPlugins", skip_serializing_if = "Option::is_none")]
    pub csi_controller_plugins: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "CSINodePlugins", skip_serializing_if = "Option::is_none")]
    pub csi_node_plugins: Option<::std::collections::HashMap<String, crate::models::CsiInfo>>,
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "Datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "Drain", skip_serializing_if = "Option::is_none")]
    pub drain: Option<bool>,
    #[serde(rename = "DrainStrategy", skip_serializing_if = "Option::is_none")]
    pub drain_strategy: Option<crate::models::DrainStrategy>,
    #[serde(rename = "Drivers", skip_serializing_if = "Option::is_none")]
    pub drivers: Option<::std::collections::HashMap<String, crate::models::DriverInfo>>,
    #[serde(rename = "Events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::NodeEvent>>,
    #[serde(rename = "HTTPAddr", skip_serializing_if = "Option::is_none")]
    pub http_addr: Option<String>,
    #[serde(rename = "HostNetworks", skip_serializing_if = "Option::is_none")]
    pub host_networks: Option<::std::collections::HashMap<String, crate::models::HostNetworkInfo>>,
    #[serde(rename = "HostVolumes", skip_serializing_if = "Option::is_none")]
    pub host_volumes: Option<::std::collections::HashMap<String, crate::models::HostVolumeInfo>>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastDrain", skip_serializing_if = "Option::is_none")]
    pub last_drain: Option<crate::models::DrainMetadata>,
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeClass", skip_serializing_if = "Option::is_none")]
    pub node_class: Option<String>,
    #[serde(rename = "NodeResources", skip_serializing_if = "Option::is_none")]
    pub node_resources: Option<crate::models::NodeResources>,
    #[serde(rename = "Reserved", skip_serializing_if = "Option::is_none")]
    pub reserved: Option<crate::models::Resources>,
    #[serde(rename = "ReservedResources", skip_serializing_if = "Option::is_none")]
    pub reserved_resources: Option<crate::models::NodeReservedResources>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<crate::models::Resources>,
    #[serde(rename = "SchedulingEligibility", skip_serializing_if = "Option::is_none")]
    pub scheduling_eligibility: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "StatusUpdatedAt", skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<i64>,
    #[serde(rename = "TLSEnabled", skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
}
