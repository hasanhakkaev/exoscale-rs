# CreateDbaasServicePgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pgbouncer_settings** | Option<[**models::JsonSchemaPgbouncer**](JsonSchemaPgbouncer.md)> |  | [optional]
**backup_schedule** | Option<[**models::UpdateDbaasServiceMysqlRequestBackupSchedule**](UpdateDbaasServiceMysqlRequestBackupSchedule.md)> |  | [optional]
**variant** | Option<[**models::EnumPgVariant**](EnumPgVariant.md)> |  | [optional]
**integrations** | Option<[**Vec<models::CreateDbaasServiceMysqlRequestIntegrationsInner>**](CreateDbaasServiceMysqlRequestIntegrationsInner.md)> | Service integrations to be enabled when creating the service. | [optional]
**timescaledb_settings** | Option<[**models::JsonSchemaTimescaledb**](JsonSchemaTimescaledb.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**synchronous_replication** | Option<[**models::EnumPgSynchronousReplication**](EnumPgSynchronousReplication.md)> |  | [optional]
**recovery_backup_time** | Option<**String**> | ISO time of a backup to recover from for services that support arbitrary times | [optional]
**pglookout_settings** | Option<[**models::JsonSchemaPglookout**](JsonSchemaPglookout.md)> |  | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](UpdateDbaasServiceMysqlRequestMaintenance.md)> |  | [optional]
**admin_username** | Option<**String**> | Custom username for admin user. This must be set only when a new service is being created. | [optional]
**version** | Option<[**models::DbaasPgTargetVersions**](DbaasPgTargetVersions.md)> |  | [optional]
**plan** | **String** | Subscription plan | 
**work_mem** | Option<**u64**> | Sets the maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files, in MB. Default is 1MB + 0.075% of total RAM (up to 32MB). | [optional]
**shared_buffers_percentage** | Option<**u64**> | Percentage of total RAM that the database server uses for shared memory buffers. Valid range is 20-60 (float), which corresponds to 20% - 60%. This setting adjusts the shared_buffers configuration value. | [optional]
**pg_settings** | Option<[**models::JsonSchemaPg**](JsonSchemaPg.md)> |  | [optional]
**admin_password** | Option<**String**> | Custom password for admin user. Defaults to random string. This must be set only when a new service is being created. | [optional]
**migration** | Option<[**models::UpdateDbaasServiceMysqlRequestMigration**](UpdateDbaasServiceMysqlRequestMigration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


