/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */ /// DbaasServiceOpensearchConnectionInfo : Opensearch connection information properties
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceOpensearchConnectionInfo {
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Vec<String>>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "dashboard-uri", skip_serializing_if = "Option::is_none")]
    pub dashboard_uri: Option<String>,
}

impl DbaasServiceOpensearchConnectionInfo {
    /// Opensearch connection information properties
    pub fn new() -> DbaasServiceOpensearchConnectionInfo {
        DbaasServiceOpensearchConnectionInfo {
            uri: None,
            username: None,
            password: None,
            dashboard_uri: None,
        }
    }
}
