use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServiceCheck {
    #[serde(rename = "AddressMode", skip_serializing_if = "Option::is_none")]
    pub address_mode: Option<String>,
    #[serde(rename = "Advertise", skip_serializing_if = "Option::is_none")]
    pub advertise: Option<String>,
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "CheckRestart", skip_serializing_if = "Option::is_none")]
    pub check_restart: Option<crate::models::CheckRestart>,
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "Expose", skip_serializing_if = "Option::is_none")]
    pub expose: Option<bool>,
    #[serde(rename = "FailuresBeforeCritical", skip_serializing_if = "Option::is_none")]
    pub failures_before_critical: Option<i32>,
    #[serde(rename = "GRPCService", skip_serializing_if = "Option::is_none")]
    pub grpc_service: Option<String>,
    #[serde(rename = "GRPCUseTLS", skip_serializing_if = "Option::is_none")]
    pub grpc_use_tls: Option<bool>,
    #[serde(rename = "Header", skip_serializing_if = "Option::is_none")]
    pub header: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "InitialStatus", skip_serializing_if = "Option::is_none")]
    pub initial_status: Option<String>,
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "Method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OnUpdate", skip_serializing_if = "Option::is_none")]
    pub on_update: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "PortLabel", skip_serializing_if = "Option::is_none")]
    pub port_label: Option<String>,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SuccessBeforePassing", skip_serializing_if = "Option::is_none")]
    pub success_before_passing: Option<i32>,
    #[serde(rename = "TLSSkipVerify", skip_serializing_if = "Option::is_none")]
    pub tls_skip_verify: Option<bool>,
    #[serde(rename = "TaskName", skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}
