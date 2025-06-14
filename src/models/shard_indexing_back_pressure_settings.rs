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
pub struct ShardIndexingBackPressureSettings {
    #[serde(rename = "primary_parameter", skip_serializing_if = "Option::is_none")]
    pub primary_parameter: Option<Box<models::PrimaryParameter>>,
    #[serde(rename = "operating_factor", skip_serializing_if = "Option::is_none")]
    pub operating_factor: Option<Box<models::OperatingFactor>>,
    /// Run shard indexing backpressure in shadow mode or enforced mode. In shadow mode (value set as false), shard indexing backpressure tracks all granular-level metrics, but it doesn’t actually reject any indexing requests. In enforced mode (value set as true), shard indexing backpressure rejects any requests to the cluster that might cause a dip in its performance. Default is false
    #[serde(rename = "enforced", skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
    /// Enable or disable shard indexing backpressure. Default is false
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ShardIndexingBackPressureSettings {
    pub fn new() -> ShardIndexingBackPressureSettings {
        ShardIndexingBackPressureSettings {
            primary_parameter: None,
            operating_factor: None,
            enforced: None,
            enabled: None,
        }
    }
}

