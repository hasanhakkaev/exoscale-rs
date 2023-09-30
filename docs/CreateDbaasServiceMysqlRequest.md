# CreateDbaasServiceMysqlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_schedule** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestBackupSchedule**](update_dbaas_service_mysql_request_backup_schedule.md)> |  | [optional]
**integrations** | Option<[**Vec<crate::models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](create_dbaas_service_mysql_request_integrations_inner.md)> | Service integrations to be enabled when creating the service. | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**recovery_backup_time** | Option<**String**> | ISO time of a backup to recover from for services that support arbitrary times | [optional]
**mysql_settings** | Option<[**serde_json::Value**](.md)> | MySQL-specific settings | [optional]
**maintenance** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**admin_username** | Option<**String**> | Custom username for admin user. This must be set only when a new service is being created. | [optional]
**version** | Option<**String**> | MySQL major version | [optional]
**plan** | **String** | Subscription plan | 
**admin_password** | Option<**String**> | Custom password for admin user. Defaults to random string. This must be set only when a new service is being created. | [optional]
**migration** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMigration**](update_dbaas_service_mysql_request_migration.md)> |  | [optional]
**binlog_retention_period** | Option<**i64**> | The minimum amount of time in seconds to keep binlog entries before deletion. This may be extended for services that require binlog entries for longer than the default for example if using the MySQL Debezium Kafka connector. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


