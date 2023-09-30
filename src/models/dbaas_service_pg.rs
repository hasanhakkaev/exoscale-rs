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
pub struct DbaasServicePg {
    /// PGBouncer connection pooling settings
    #[serde(rename = "pgbouncer-settings", skip_serializing_if = "Option::is_none")]
    pub pgbouncer_settings: Option<serde_json::Value>,
    /// Service last update timestamp (ISO 8601)
    #[serde(rename = "updated-at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Number of service nodes in the active plan
    #[serde(rename = "node-count", skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i64>,
    #[serde(rename = "connection-info", skip_serializing_if = "Option::is_none")]
    pub connection_info: Option<Box<crate::models::DbaasServicePgConnectionInfo>>,
    #[serde(rename = "backup-schedule", skip_serializing_if = "Option::is_none")]
    pub backup_schedule: Option<Box<crate::models::DbaasServiceMysqlBackupSchedule>>,
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
    /// PostgreSQL PGBouncer connection pools
    #[serde(rename = "connection-pools", skip_serializing_if = "Option::is_none")]
    pub connection_pools: Option<Vec<crate::models::DbaasServicePgConnectionPoolsInner>>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::EnumServiceState>,
    /// TimescaleDB extension configuration values
    #[serde(rename = "timescaledb-settings", skip_serializing_if = "Option::is_none")]
    pub timescaledb_settings: Option<serde_json::Value>,
    /// List of PostgreSQL databases
    #[serde(rename = "databases", skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<String>>,
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
    pub components: Option<Vec<crate::models::DbaasServiceMysqlComponentsInner>>,
    #[serde(rename = "synchronous-replication", skip_serializing_if = "Option::is_none")]
    pub synchronous_replication: Option<crate::models::EnumPgSynchronousReplication>,
    /// PGLookout settings
    #[serde(rename = "pglookout-settings", skip_serializing_if = "Option::is_none")]
    pub pglookout_settings: Option<serde_json::Value>,
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
    /// PostgreSQL version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Service creation timestamp (ISO 8601)
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Subscription plan
    #[serde(rename = "plan")]
    pub plan: String,
    /// Sets the maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files, in MB. Default is 1MB + 0.075% of total RAM (up to 32MB).
    #[serde(rename = "work-mem", skip_serializing_if = "Option::is_none")]
    pub work_mem: Option<i64>,
    /// Percentage of total RAM that the database server uses for shared memory buffers. Valid range is 20-60 (float), which corresponds to 20% - 60%. This setting adjusts the shared_buffers configuration value.
    #[serde(rename = "shared-buffers-percentage", skip_serializing_if = "Option::is_none")]
    pub shared_buffers_percentage: Option<i64>,
    /// PostgreSQL-specific settings
    #[serde(rename = "pg-settings", skip_serializing_if = "Option::is_none")]
    pub pg_settings: Option<serde_json::Value>,
    /// Maximum number of connections allowed to an instance
    #[serde(rename = "max-connections", skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// List of service users
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::DbaasServicePgUsersInner>>,
}

impl DbaasServicePg {
    pub fn new(name: String, r#type: String, plan: String) -> DbaasServicePg {
        DbaasServicePg {
            pgbouncer_settings: None,
            updated_at: None,
            node_count: None,
            connection_info: None,
            backup_schedule: None,
            node_cpu_count: None,
            integrations: None,
            zone: None,
            node_states: None,
            name,
            connection_pools: None,
            r#type,
            state: None,
            timescaledb_settings: None,
            databases: None,
            ip_filter: None,
            backups: None,
            termination_protection: None,
            notifications: None,
            components: None,
            synchronous_replication: None,
            pglookout_settings: None,
            maintenance: None,
            disk_size: None,
            node_memory: None,
            uri: None,
            uri_params: None,
            version: None,
            created_at: None,
            plan,
            work_mem: None,
            shared_buffers_percentage: None,
            pg_settings: None,
            max_connections: None,
            users: None,
        }
    }
}

