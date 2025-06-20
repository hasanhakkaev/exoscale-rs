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
pub struct KubeletImageGc {
    #[serde(rename = "high-threshold", skip_serializing_if = "Option::is_none")]
    pub high_threshold: Option<u64>,
    #[serde(rename = "low-threshold", skip_serializing_if = "Option::is_none")]
    pub low_threshold: Option<u64>,
    #[serde(rename = "min-age", skip_serializing_if = "Option::is_none")]
    pub min_age: Option<String>,
}

impl KubeletImageGc {
    /// Kubelet image GC options
    pub fn new() -> KubeletImageGc {
        KubeletImageGc {
            high_threshold: None,
            low_threshold: None,
            min_age: None,
        }
    }
}

