use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SampledValue {
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "Max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Mean", skip_serializing_if = "Option::is_none")]
    pub mean: Option<f64>,
    #[serde(rename = "Min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(rename = "Stddev", skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f64>,
    #[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
}


