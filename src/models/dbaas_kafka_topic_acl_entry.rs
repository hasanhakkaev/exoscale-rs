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
pub struct DbaasKafkaTopicAclEntry {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Kafka username or username pattern
    #[serde(rename = "username")]
    pub username: String,
    /// Kafka topic name or pattern
    #[serde(rename = "topic")]
    pub topic: String,
    /// Kafka permission
    #[serde(rename = "permission")]
    pub permission: Permission,
}

impl DbaasKafkaTopicAclEntry {
    pub fn new(username: String, topic: String, permission: Permission) -> DbaasKafkaTopicAclEntry {
        DbaasKafkaTopicAclEntry {
            id: None,
            username,
            topic,
            permission,
        }
    }
}

/// Kafka permission
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permission {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "readwrite")]
    Readwrite,
    #[serde(rename = "write")]
    Write,
}

impl Default for Permission {
    fn default() -> Permission {
        Self::Admin
    }
}
