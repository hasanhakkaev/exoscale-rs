/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceRedisUsersInnerAccessControl {
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(rename = "keys", skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

impl DbaasServiceRedisUsersInnerAccessControl {
    pub fn new() -> DbaasServiceRedisUsersInnerAccessControl {
        DbaasServiceRedisUsersInnerAccessControl {
            categories: None,
            channels: None,
            commands: None,
            keys: None,
        }
    }
}


