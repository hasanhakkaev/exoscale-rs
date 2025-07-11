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
pub struct DbaasMigrationStatusDetailsInner {
    /// Migrated db name (PG) or number (Valkey)
    #[serde(rename = "dbname", skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
    /// Error message in case that migration has failed
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Migration method
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::EnumMigrationStatus>,
}

impl DbaasMigrationStatusDetailsInner {
    pub fn new() -> DbaasMigrationStatusDetailsInner {
        DbaasMigrationStatusDetailsInner {
            dbname: None,
            error: None,
            method: None,
            status: None,
        }
    }
}

