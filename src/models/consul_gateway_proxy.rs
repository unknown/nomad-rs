use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConsulGatewayProxy {
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "ConnectTimeout", skip_serializing_if = "Option::is_none")]
    pub connect_timeout: Option<i64>,
    #[serde(rename = "EnvoyDNSDiscoveryType", skip_serializing_if = "Option::is_none")]
    pub envoy_dns_discovery_type: Option<String>,
    #[serde(rename = "EnvoyGatewayBindAddresses", skip_serializing_if = "Option::is_none")]
    pub envoy_gateway_bind_addresses: Option<::std::collections::HashMap<String, crate::models::ConsulGatewayBindAddress>>,
    #[serde(rename = "EnvoyGatewayBindTaggedAddresses", skip_serializing_if = "Option::is_none")]
    pub envoy_gateway_bind_tagged_addresses: Option<bool>,
    #[serde(rename = "EnvoyGatewayNoDefaultBind", skip_serializing_if = "Option::is_none")]
    pub envoy_gateway_no_default_bind: Option<bool>,
}

