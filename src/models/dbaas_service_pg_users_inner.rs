/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServicePgUsersInner : List of service users



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServicePgUsersInner {
    /// Account type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Account username
    #[serde(rename = "username")]
    pub username: String,
    /// Account password. A missing field indicates a user overridden password.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "allow-replication", skip_serializing_if = "Option::is_none")]
    pub allow_replication: Option<bool>,
}

impl DbaasServicePgUsersInner {
    /// List of service users
    pub fn new(r#type: String, username: String) -> DbaasServicePgUsersInner {
        DbaasServicePgUsersInner {
            r#type,
            username,
            password: None,
            allow_replication: None,
        }
    }
}


