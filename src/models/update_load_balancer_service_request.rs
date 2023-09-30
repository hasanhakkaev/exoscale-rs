/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLoadBalancerServiceRequest {
    /// Load Balancer Service name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Load Balancer Service description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Network traffic protocol
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// Load balancing strategy
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    /// Port exposed on the Load Balancer's public IP
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Port on which the network traffic will be forwarded to on the receiving instance
    #[serde(rename = "target-port", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i64>,
    #[serde(rename = "healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<crate::models::LoadBalancerServiceHealthcheck>>,
}

impl UpdateLoadBalancerServiceRequest {
    pub fn new() -> UpdateLoadBalancerServiceRequest {
        UpdateLoadBalancerServiceRequest {
            name: None,
            description: None,
            protocol: None,
            strategy: None,
            port: None,
            target_port: None,
            healthcheck: None,
        }
    }
}

/// Network traffic protocol
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}
/// Load balancing strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "round-robin")]
    RoundRobin,
    #[serde(rename = "source-hash")]
    SourceHash,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::RoundRobin
    }
}
