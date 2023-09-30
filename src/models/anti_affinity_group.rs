/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// AntiAffinityGroup : Anti-affinity Group



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AntiAffinityGroup {
    /// Anti-affinity Group ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Anti-affinity Group name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Anti-affinity Group description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Anti-affinity Group instances
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<crate::models::Instance>>,
}

impl AntiAffinityGroup {
    /// Anti-affinity Group
    pub fn new() -> AntiAffinityGroup {
        AntiAffinityGroup {
            id: None,
            name: None,
            description: None,
            instances: None,
        }
    }
}


