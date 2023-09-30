/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// GetDbaasSettingsKafka200ResponseSettingsKafka : Kafka broker configuration values



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDbaasSettingsKafka200ResponseSettingsKafka {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl GetDbaasSettingsKafka200ResponseSettingsKafka {
    /// Kafka broker configuration values
    pub fn new() -> GetDbaasSettingsKafka200ResponseSettingsKafka {
        GetDbaasSettingsKafka200ResponseSettingsKafka {
            properties: None,
            additional_properties: None,
            r#type: None,
            title: None,
        }
    }
}


