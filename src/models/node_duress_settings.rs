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
pub struct NodeDuressSettings {
    /// The CPU usage threshold (as a percentage) required for a node to be considered to be under duress. Default is 0.9
    #[serde(rename = "cpu_threshold", skip_serializing_if = "Option::is_none")]
    pub cpu_threshold: Option<f64>,
    /// The heap usage threshold (as a percentage) required for a node to be considered to be under duress. Default is 0.7
    #[serde(rename = "heap_threshold", skip_serializing_if = "Option::is_none")]
    pub heap_threshold: Option<f64>,
    /// The number of successive limit breaches after which the node is considered to be under duress. Default is 3
    #[serde(rename = "num_successive_breaches", skip_serializing_if = "Option::is_none")]
    pub num_successive_breaches: Option<u32>,
}

impl NodeDuressSettings {
    pub fn new() -> NodeDuressSettings {
        NodeDuressSettings {
            cpu_threshold: None,
            heap_threshold: None,
            num_successive_breaches: None,
        }
    }
}

