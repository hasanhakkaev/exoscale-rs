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


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnumMigrationStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "syncing")]
    Syncing,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "done")]
    Done,

}

impl std::fmt::Display for EnumMigrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Syncing => write!(f, "syncing"),
            Self::Failed => write!(f, "failed"),
            Self::Done => write!(f, "done"),
        }
    }
}

impl Default for EnumMigrationStatus {
    fn default() -> EnumMigrationStatus {
        Self::Running
    }
}

