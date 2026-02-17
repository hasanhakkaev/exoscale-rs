# CreateDbaasServiceMysqlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backup_schedule** | Option<[**models::UpdateDbaasServiceMysqlRequestBackupSchedule**](UpdateDbaasServiceMysqlRequestBackupSchedule.md)> |  | [optional]
**integrations** | Option<[**Vec<models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](CreateDbaasServiceMysqlRequestIntegrationsInner.md)> | Service integrations to be enabled when creating the service. | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**recovery_backup_time** | Option<**String**> | ISO time of a backup to recover from for services that support arbitrary times | [optional]
**mysql_settings** | Option<[**models::JsonSchemaMysql**](JsonSchemaMysql.md)> |  | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](UpdateDbaasServiceMysqlRequestMaintenance.md)> |  | [optional]
**admin_username** | Option<**String**> | Custom username for admin user. This must be set only when a new service is being created. | [optional]
**version** | Option<**String**> | MySQL major version | [optional]
**plan** | **String** | Subscription plan | 
**admin_password** | Option<**String**> |  | [optional]
**migration** | Option<[**models::UpdateDbaasServiceMysqlRequestMigration**](UpdateDbaasServiceMysqlRequestMigration.md)> |  | [optional]
**binlog_retention_period** | Option<**u64**> | The minimum amount of time in seconds to keep binlog entries before deletion. This may be extended for services that require binlog entries for longer than the default for example if using the MySQL Debezium Kafka connector. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


