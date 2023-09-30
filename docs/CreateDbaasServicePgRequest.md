# CreateDbaasServicePgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pgbouncer_settings** | Option<[**serde_json::Value**](.md)> | PGBouncer connection pooling settings | [optional]
**backup_schedule** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestBackupSchedule**](update_dbaas_service_mysql_request_backup_schedule.md)> |  | [optional]
**variant** | Option<[**crate::models::EnumPgVariant**](enum-pg-variant.md)> |  | [optional]
**integrations** | Option<[**Vec<crate::models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](create_dbaas_service_mysql_request_integrations_inner.md)> | Service integrations to be enabled when creating the service. | [optional]
**timescaledb_settings** | Option<[**serde_json::Value**](.md)> | TimescaleDB extension configuration values | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**synchronous_replication** | Option<[**crate::models::EnumPgSynchronousReplication**](enum-pg-synchronous-replication.md)> |  | [optional]
**recovery_backup_time** | Option<**String**> | ISO time of a backup to recover from for services that support arbitrary times | [optional]
**pglookout_settings** | Option<[**serde_json::Value**](.md)> | PGLookout settings | [optional]
**maintenance** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**admin_username** | Option<**String**> | Custom username for admin user. This must be set only when a new service is being created. | [optional]
**version** | Option<**String**> | PostgreSQL major version | [optional]
**plan** | **String** | Subscription plan | 
**work_mem** | Option<**i64**> | Sets the maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files, in MB. Default is 1MB + 0.075% of total RAM (up to 32MB). | [optional]
**shared_buffers_percentage** | Option<**i64**> | Percentage of total RAM that the database server uses for shared memory buffers. Valid range is 20-60 (float), which corresponds to 20% - 60%. This setting adjusts the shared_buffers configuration value. | [optional]
**pg_settings** | Option<[**serde_json::Value**](.md)> | PostgreSQL-specific settings | [optional]
**admin_password** | Option<**String**> | Custom password for admin user. Defaults to random string. This must be set only when a new service is being created. | [optional]
**migration** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMigration**](update_dbaas_service_mysql_request_migration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


