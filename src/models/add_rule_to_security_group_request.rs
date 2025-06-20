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
pub struct AddRuleToSecurityGroupRequest {
    /// Network flow direction to match
    #[serde(rename = "flow-direction")]
    pub flow_direction: FlowDirection,
    /// Security Group rule description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// CIDR-formatted network allowed
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(rename = "security-group", skip_serializing_if = "Option::is_none")]
    pub security_group: Option<Box<models::SecurityGroupResource>>,
    /// Network protocol
    #[serde(rename = "protocol")]
    pub protocol: Protocol,
    #[serde(rename = "icmp", skip_serializing_if = "Option::is_none")]
    pub icmp: Option<Box<models::AddRuleToSecurityGroupRequestIcmp>>,
    /// Start port of the range
    #[serde(rename = "start-port", skip_serializing_if = "Option::is_none")]
    pub start_port: Option<u64>,
    /// End port of the range
    #[serde(rename = "end-port", skip_serializing_if = "Option::is_none")]
    pub end_port: Option<u64>,
}

impl AddRuleToSecurityGroupRequest {
    pub fn new(flow_direction: FlowDirection, protocol: Protocol) -> AddRuleToSecurityGroupRequest {
        AddRuleToSecurityGroupRequest {
            flow_direction,
            description: None,
            network: None,
            security_group: None,
            protocol,
            icmp: None,
            start_port: None,
            end_port: None,
        }
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

