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
pub struct CreateDbaasServiceMysqlRequest {
    #[serde(rename = "backup-schedule", skip_serializing_if = "Option::is_none")]
    pub backup_schedule: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestBackupSchedule>>,
    /// Service integrations to be enabled when creating the service.
    #[serde(rename = "integrations", skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<crate::models::CreateDbaasServiceMysqlRequestIntegrationsInner>>,
    /// Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16'
    #[serde(rename = "ip-filter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    /// Service is protected against termination and powering off
    #[serde(rename = "termination-protection", skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    #[serde(rename = "fork-from-service", skip_serializing_if = "Option::is_none")]
    pub fork_from_service: Option<String>,
    /// ISO time of a backup to recover from for services that support arbitrary times
    #[serde(rename = "recovery-backup-time", skip_serializing_if = "Option::is_none")]
    pub recovery_backup_time: Option<String>,
    /// MySQL-specific settings
    #[serde(rename = "mysql-settings", skip_serializing_if = "Option::is_none")]
    pub mysql_settings: Option<serde_json::Value>,
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMaintenance>>,
    /// Custom username for admin user. This must be set only when a new service is being created.
    #[serde(rename = "admin-username", skip_serializing_if = "Option::is_none")]
    pub admin_username: Option<String>,
    /// MySQL major version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Subscription plan
    #[serde(rename = "plan")]
    pub plan: String,
    /// Custom password for admin user. Defaults to random string. This must be set only when a new service is being created.
    #[serde(rename = "admin-password", skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[serde(rename = "migration", skip_serializing_if = "Option::is_none")]
    pub migration: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMigration>>,
    /// The minimum amount of time in seconds to keep binlog entries before deletion. This may be extended for services that require binlog entries for longer than the default for example if using the MySQL Debezium Kafka connector.
    #[serde(rename = "binlog-retention-period", skip_serializing_if = "Option::is_none")]
    pub binlog_retention_period: Option<i64>,
}

impl CreateDbaasServiceMysqlRequest {
    pub fn new(plan: String) -> CreateDbaasServiceMysqlRequest {
        CreateDbaasServiceMysqlRequest {
            backup_schedule: None,
            integrations: None,
            ip_filter: None,
            termination_protection: None,
            fork_from_service: None,
            recovery_backup_time: None,
            mysql_settings: None,
            maintenance: None,
            admin_username: None,
            version: None,
            plan,
            admin_password: None,
            migration: None,
            binlog_retention_period: None,
        }
    }
}


