/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServiceBackup : List of backups for the service

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceBackup {
    /// Internal name of this backup
    #[serde(rename = "backup_name")]
    pub backup_name: String,
    /// Backup timestamp (ISO 8601)
    #[serde(rename = "backup-time")]
    pub backup_time: String,
    /// Backup's original size before compression
    #[serde(rename = "data-size")]
    pub data_size: i64,
}

impl DbaasServiceBackup {
    /// List of backups for the service
    pub fn new(backup_name: String, backup_time: String, data_size: i64) -> DbaasServiceBackup {
        DbaasServiceBackup {
            backup_name,
            backup_time,
            data_size,
        }
    }
}
