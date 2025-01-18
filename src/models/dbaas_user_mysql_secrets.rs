/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */ /// DbaasUserMysqlSecrets : MySQL User secrets
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasUserMysqlSecrets {
    /// MySQL username
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// MySQL password
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl DbaasUserMysqlSecrets {
    /// MySQL User secrets
    pub fn new() -> DbaasUserMysqlSecrets {
        DbaasUserMysqlSecrets {
            username: None,
            password: None,
        }
    }
}
