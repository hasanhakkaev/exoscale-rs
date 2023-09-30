/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServiceKafkaConnectionInfo : Kafka connection information properties



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceKafkaConnectionInfo {
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    #[serde(rename = "access-cert", skip_serializing_if = "Option::is_none")]
    pub access_cert: Option<String>,
    #[serde(rename = "access-key", skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    #[serde(rename = "connect-uri", skip_serializing_if = "Option::is_none")]
    pub connect_uri: Option<String>,
    #[serde(rename = "rest-uri", skip_serializing_if = "Option::is_none")]
    pub rest_uri: Option<String>,
    #[serde(rename = "registry-uri", skip_serializing_if = "Option::is_none")]
    pub registry_uri: Option<String>,
}

impl DbaasServiceKafkaConnectionInfo {
    /// Kafka connection information properties
    pub fn new() -> DbaasServiceKafkaConnectionInfo {
        DbaasServiceKafkaConnectionInfo {
            nodes: None,
            access_cert: None,
            access_key: None,
            connect_uri: None,
            rest_uri: None,
            registry_uri: None,
        }
    }
}


