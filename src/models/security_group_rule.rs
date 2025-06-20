/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityGroupRule {
    /// Security Group rule description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Start port of the range
    #[serde(rename = "start-port", skip_serializing_if = "Option::is_none")]
    pub start_port: Option<u64>,
    /// Network protocol
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "icmp", skip_serializing_if = "Option::is_none")]
    pub icmp: Option<Box<models::SecurityGroupRuleIcmp>>,
    /// End port of the range
    #[serde(rename = "end-port", skip_serializing_if = "Option::is_none")]
    pub end_port: Option<u64>,
    #[serde(rename = "security-group", skip_serializing_if = "Option::is_none")]
    pub security_group: Option<Box<models::SecurityGroupResource>>,
    /// Security Group rule ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// CIDR-formatted network allowed
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// Network flow direction to match
    #[serde(rename = "flow-direction", skip_serializing_if = "Option::is_none")]
    pub flow_direction: Option<FlowDirection>,
}

impl SecurityGroupRule {
    /// Security Group rule
    pub fn new() -> SecurityGroupRule {
        SecurityGroupRule {
            description: None,
            start_port: None,
            protocol: None,
            icmp: None,
            end_port: None,
            security_group: None,
            id: None,
            network: None,
            flow_direction: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "esp")]
    Esp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "gre")]
    Gre,
    #[serde(rename = "ah")]
    Ah,
    #[serde(rename = "ipip")]
    Ipip,
    #[serde(rename = "icmpv6")]
    Icmpv6,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowDirection {
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "egress")]
    Egress,
}

impl Default for FlowDirection {
    fn default() -> FlowDirection {
        Self::Ingress
    }
}

