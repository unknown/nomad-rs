use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricsSummary {
    #[serde(rename = "Counters", skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<crate::models::SampledValue>>,
    #[serde(rename = "Gauges", skip_serializing_if = "Option::is_none")]
    pub gauges: Option<Vec<crate::models::GaugeValue>>,
    #[serde(rename = "Points", skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<crate::models::PointValue>>,
    #[serde(rename = "Samples", skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<crate::models::SampledValue>>,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

