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
pub struct GetDbaasSettingsKafka200ResponseSettings {
    #[serde(rename = "kafka", skip_serializing_if = "Option::is_none")]
    pub kafka: Option<Box<models::GetDbaasSettingsKafka200ResponseSettingsKafka>>,
    #[serde(rename = "kafka-connect", skip_serializing_if = "Option::is_none")]
    pub kafka_connect: Option<Box<models::GetDbaasSettingsKafka200ResponseSettingsKafkaConnect>>,
    #[serde(rename = "kafka-rest", skip_serializing_if = "Option::is_none")]
    pub kafka_rest: Option<Box<models::GetDbaasSettingsKafka200ResponseSettingsKafkaRest>>,
    #[serde(rename = "schema-registry", skip_serializing_if = "Option::is_none")]
    pub schema_registry:
        Option<Box<models::GetDbaasSettingsKafka200ResponseSettingsSchemaRegistry>>,
}

impl GetDbaasSettingsKafka200ResponseSettings {
    pub fn new() -> GetDbaasSettingsKafka200ResponseSettings {
        GetDbaasSettingsKafka200ResponseSettings {
            kafka: None,
            kafka_connect: None,
            kafka_rest: None,
            schema_registry: None,
        }
    }
}
