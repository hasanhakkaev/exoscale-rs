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
pub struct DbaasServiceRedis {
    /// Service last update timestamp (ISO 8601)
    #[serde(rename = "updated-at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Number of service nodes in the active plan
    #[serde(rename = "node-count", skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i64>,
    #[serde(rename = "connection-info", skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<Box<crate::models::DbaasServiceRedisConnectionInfo>>,
    /// Number of CPUs for each node
    #[serde(rename = "node-cpu-count", skip_serializing_if = "Option::is_none")]
    pub node_cpu_count: Option<i64>,
    /// Service integrations
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<crate::models::DbaasIntegration>>,
    /// The zone where the service is running
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// State of individual service nodes
    #[serde(rename = "node-states", skip_serializing_if = "Option::is_none")]
    pub node_states: Option<Vec<crate::models::DbaasNodeState>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Redis-specific settings
    #[serde(rename = "redis-settings", skip_serializing_if = "Option::is_none")]
    pub redis_settings: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::EnumServiceState>,
    /// Allowed CIDR address blocks for incoming connections
    #[serde(rename = "ip-filter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    /// List of backups for the service
    #[serde(rename = "backups", skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<crate::models::DbaasServiceBackup>>,
    /// Service is protected against termination and powering off
    #[serde(rename = "termination-protection", skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Service notifications
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<crate::models::DbaasServiceNotification>>,
    /// Service component information objects
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::DbaasServiceRedisComponentsInner>>,
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<crate::models::DbaasServiceMaintenance>>,
    /// TODO UNIT disk space for data storage
    #[serde(rename = "disk-size", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// TODO UNIT of memory for each node
    #[serde(rename = "node-memory", skip_serializing_if = "Option::is_none")]
    pub node_memory: Option<i64>,
    /// URI for connecting to the service (may be absent)
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// service_uri parameterized into key-value pairs
    #[serde(rename = "uri-params", skip_serializing_if = "Option::is_none")]
    pub uri_params: Option<serde_json::Value>,
    /// Redis version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Service creation timestamp (ISO 8601)
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Subscription plan
    #[serde(rename = "plan")]
    pub plan: String,
    /// List of service users
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::DbaasServiceRedisUsersInner>>,
}

impl DbaasServiceRedis {
    pub fn new(name: String, r#type: String, plan: String) -> DbaasServiceRedis {
        DbaasServiceRedis {
            updated_at: None,
            node_count: None,
            connection_info: None,
            node_cpu_count: None,
            integrations: None,
            zone: None,
            node_states: None,
            name,
            redis_settings: None,
            r#type,
            state: None,
            ip_filter: None,
            backups: None,
            termination_protection: None,
            notifications: None,
            components: None,
            maintenance: None,
            disk_size: None,
            node_memory: None,
            uri: None,
            uri_params: None,
            version: None,
            created_at: None,
            plan,
            users: None,
        }
    }
}


