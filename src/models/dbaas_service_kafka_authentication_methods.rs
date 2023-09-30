/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServiceKafkaAuthenticationMethods : Kafka authentication methods



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceKafkaAuthenticationMethods {
    /// Whether certificate/SSL authentication is enabled
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<bool>,
    /// Whether SASL authentication is enabled
    #[serde(rename = "sasl", skip_serializing_if = "Option::is_none")]
    pub sasl: Option<bool>,
}

impl DbaasServiceKafkaAuthenticationMethods {
    /// Kafka authentication methods
    pub fn new() -> DbaasServiceKafkaAuthenticationMethods {
        DbaasServiceKafkaAuthenticationMethods {
            certificate: None,
            sasl: None,
        }
    }
}

