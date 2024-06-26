use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CsiVolume {
    #[serde(rename = "AccessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<String>,
    #[serde(rename = "Allocations", skip_serializing_if = "Option::is_none")]
    pub allocations: Option<Vec<crate::models::AllocationListStub>>,
    #[serde(rename = "AttachmentMode", skip_serializing_if = "Option::is_none")]
    pub attachment_mode: Option<String>,
    #[serde(rename = "Capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[serde(rename = "CloneID", skip_serializing_if = "Option::is_none")]
    pub clone_id: Option<String>,
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "ControllerRequired", skip_serializing_if = "Option::is_none")]
    pub controller_required: Option<bool>,
    #[serde(rename = "ControllersExpected", skip_serializing_if = "Option::is_none")]
    pub controllers_expected: Option<i32>,
    #[serde(rename = "ControllersHealthy", skip_serializing_if = "Option::is_none")]
    pub controllers_healthy: Option<i32>,
    #[serde(rename = "CreateIndex", skip_serializing_if = "Option::is_none")]
    pub create_index: Option<i32>,
    #[serde(rename = "ExternalID", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ModifyIndex", skip_serializing_if = "Option::is_none")]
    pub modify_index: Option<i32>,
    #[serde(rename = "MountOptions", skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<crate::models::CsiMountOptions>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NodesExpected", skip_serializing_if = "Option::is_none")]
    pub nodes_expected: Option<i32>,
    #[serde(rename = "NodesHealthy", skip_serializing_if = "Option::is_none")]
    pub nodes_healthy: Option<i32>,
    #[serde(rename = "Parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "PluginID", skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(rename = "Provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "ProviderVersion", skip_serializing_if = "Option::is_none")]
    pub provider_version: Option<String>,
    #[serde(rename = "ReadAllocs", skip_serializing_if = "Option::is_none")]
    pub read_allocs: Option<::std::collections::HashMap<String, crate::models::Allocation>>,
    #[serde(rename = "RequestedCapabilities", skip_serializing_if = "Option::is_none")]
    pub requested_capabilities: Option<Vec<crate::models::CsiVolumeCapability>>,
    #[serde(rename = "RequestedCapacityMax", skip_serializing_if = "Option::is_none")]
    pub requested_capacity_max: Option<i64>,
    #[serde(rename = "RequestedCapacityMin", skip_serializing_if = "Option::is_none")]
    pub requested_capacity_min: Option<i64>,
    #[serde(rename = "RequestedTopologies", skip_serializing_if = "Option::is_none")]
    pub requested_topologies: Option<crate::models::CsiTopologyRequest>,
    #[serde(rename = "ResourceExhausted", skip_serializing_if = "Option::is_none")]
    pub resource_exhausted: Option<String>,
    #[serde(rename = "Schedulable", skip_serializing_if = "Option::is_none")]
    pub schedulable: Option<bool>,
    #[serde(rename = "Secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "SnapshotID", skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Topologies", skip_serializing_if = "Option::is_none")]
    pub topologies: Option<Vec<crate::models::CsiTopology>>,
    #[serde(rename = "WriteAllocs", skip_serializing_if = "Option::is_none")]
    pub write_allocs: Option<::std::collections::HashMap<String, crate::models::Allocation>>,
}
