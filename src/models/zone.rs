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

/// Zone : Zone
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<models::ZoneName>,
    /// Zone API endpoint
    #[serde(rename = "api-endpoint", skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    /// Zone SOS endpoint
    #[serde(rename = "sos-endpoint", skip_serializing_if = "Option::is_none")]
    pub sos_endpoint: Option<String>,
}

impl Zone {
    /// Zone
    pub fn new() -> Zone {
        Zone {
            name: None,
            api_endpoint: None,
            sos_endpoint: None,
        }
    }
}
