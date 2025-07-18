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
pub struct GetDbaasServiceMetricsRequest {
    /// Metrics time period (default: hour)
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
}

impl GetDbaasServiceMetricsRequest {
    pub fn new() -> GetDbaasServiceMetricsRequest {
        GetDbaasServiceMetricsRequest {
            period: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Period {
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "day")]
    Day,
}

impl Default for Period {
    fn default() -> Period {
        Self::Hour
    }
}

