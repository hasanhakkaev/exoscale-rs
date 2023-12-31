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
pub struct UpdateDbaasServiceMysqlRequest {
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMaintenance>>,
    /// Subscription plan
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Service is protected against termination and powering off
    #[serde(rename = "termination-protection", skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16'
    #[serde(rename = "ip-filter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
    /// MySQL-specific settings
    #[serde(rename = "mysql-settings", skip_serializing_if = "Option::is_none")]
    pub mysql_settings: Option<serde_json::Value>,
    #[serde(rename = "migration", skip_serializing_if = "Option::is_none")]
    pub migration: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMigration>>,
    /// The minimum amount of time in seconds to keep binlog entries before deletion. This may be extended for services that require binlog entries for longer than the default for example if using the MySQL Debezium Kafka connector.
    #[serde(rename = "binlog-retention-period", skip_serializing_if = "Option::is_none")]
    pub binlog_retention_period: Option<i64>,
    #[serde(rename = "backup-schedule", skip_serializing_if = "Option::is_none")]
    pub backup_schedule: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestBackupSchedule>>,
}

impl UpdateDbaasServiceMysqlRequest {
    pub fn new() -> UpdateDbaasServiceMysqlRequest {
        UpdateDbaasServiceMysqlRequest {
            maintenance: None,
            plan: None,
            termination_protection: None,
            ip_filter: None,
            mysql_settings: None,
            migration: None,
            binlog_retention_period: None,
            backup_schedule: None,
        }
    }
}


