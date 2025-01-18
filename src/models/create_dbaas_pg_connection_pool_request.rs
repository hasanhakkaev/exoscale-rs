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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDbaasPgConnectionPoolRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "database-name")]
    pub database_name: String,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::EnumPgPoolMode>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl CreateDbaasPgConnectionPoolRequest {
    pub fn new(name: String, database_name: String) -> CreateDbaasPgConnectionPoolRequest {
        CreateDbaasPgConnectionPoolRequest {
            name,
            database_name,
            mode: None,
            size: None,
            username: None,
        }
    }
}
