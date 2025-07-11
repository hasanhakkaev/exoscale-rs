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
pub struct DbaasPlan {
    /// DBaaS plan node count
    #[serde(rename = "node-count", skip_serializing_if = "Option::is_none")]
    pub node_count: Option<u64>,
    #[serde(rename = "backup-config", skip_serializing_if = "Option::is_none")]
    pub backup_config: Option<Box<models::DbaasBackupConfig>>,
    /// DBaaS plan CPU count per node
    #[serde(rename = "node-cpu-count", skip_serializing_if = "Option::is_none")]
    pub node_cpu_count: Option<u64>,
    /// Instance family subset which the service can use
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// DBaaS plan disk space
    #[serde(rename = "disk-space", skip_serializing_if = "Option::is_none")]
    pub disk_space: Option<i64>,
    /// Requires authorization or publicly available
    #[serde(rename = "authorized", skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    /// DBaaS plan name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// DBaaS plan max memory allocated percentage
    #[serde(rename = "max-memory-percent", skip_serializing_if = "Option::is_none")]
    pub max_memory_percent: Option<u64>,
    /// Zones where the plan is available
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
    /// DBaaS plan memory count per node
    #[serde(rename = "node-memory", skip_serializing_if = "Option::is_none")]
    pub node_memory: Option<u64>,
}

impl DbaasPlan {
    /// DBaaS plan
    pub fn new() -> DbaasPlan {
        DbaasPlan {
            node_count: None,
            backup_config: None,
            node_cpu_count: None,
            family: None,
            disk_space: None,
            authorized: None,
            name: None,
            max_memory_percent: None,
            zones: None,
            node_memory: None,
        }
    }
}

