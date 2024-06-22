use serde::{Deserialize, Serialize};

use crate::models::allocation::Allocation;
use crate::models::deployment::Deployment;
use crate::models::evaluation::Evaluation;
use crate::models::job::Job;
use crate::models::node::Node;
use crate::models::service::Service;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventCollection {
    #[serde(rename = "Index")]
    pub index: i32,
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[serde(rename = "Index")]
    pub index: i32,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<EventTopic>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<EventType>,
    #[serde(rename = "Payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<EventPayload>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventPayload {
    Allocation(Box<Allocation>),
    Deployment(Box<Deployment>),
    Evaluation(Box<Evaluation>),
    Job(Box<Job>),
    Node(Box<Node>),
    Service(Box<Service>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventTopic {
    ACLToken,
    ACLPolicy,
    ACLRole,
    Job,
    Allocation,
    Deployment,
    Evaluation,
    Node,
    Service,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventType {
    ACLTokenUpserted,
    ACLTokenDeleted,
    ACLPolicyUpserted,
    ACLPolicyDeleted,
    ACLRoleUpserted,
    ACLRoleDeleted,
    AllocationCreated,
    AllocationUpdated,
    AllocationUpdateDesiredStatus,
    DeploymentStatusUpdate,
    DeploymentPromotion,
    DeploymentAllocHealth,
    EvaluationUpdated,
    JobRegistered,
    JobDeregistered,
    JobBatchDeregistered,
    NodeRegistration,
    NodeDeregistration,
    NodeEligibility,
    NodeStreamEvent,
    NodeDrain,
    NodeEvent,
    PlanResult,
    ServiceRegistration,
    ServiceDeregistration,
}