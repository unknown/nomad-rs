use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RaftServer {
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(rename = "RaftProtocol", skip_serializing_if = "Option::is_none")]
    pub raft_protocol: Option<String>,
    #[serde(rename = "Voter", skip_serializing_if = "Option::is_none")]
    pub voter: Option<bool>,
}


