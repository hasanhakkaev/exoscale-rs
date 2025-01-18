/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */
/// Manager : Resource manager
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manager {
    /// Manager ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Manager type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl Manager {
    /// Resource manager
    pub fn new() -> Manager {
        Manager {
            id: None,
            r#type: None,
        }
    }
}
/// Manager type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "sks-nodepool")]
    SksNodepool,
    #[serde(rename = "instance-pool")]
    InstancePool,
}

impl Default for Type {
    fn default() -> Type {
        Self::SksNodepool
    }
}
