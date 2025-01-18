# UpdateDbaasServicePgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pgbouncer_settings** | Option<[**models::JsonSchemaPgbouncer**](json-schema-pgbouncer.md)> |  | [optional]
**backup_schedule** | Option<[**models::UpdateDbaasServiceMysqlRequestBackupSchedule**](update_dbaas_service_mysql_request_backup_schedule.md)> |  | [optional]
**variant** | Option<[**models::EnumPgVariant**](enum-pg-variant.md)> |  | [optional]
**timescaledb_settings** | Option<[**models::JsonSchemaTimescaledb**](json-schema-timescaledb.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**synchronous_replication** | Option<[**models::EnumPgSynchronousReplication**](enum-pg-synchronous-replication.md)> |  | [optional]
**pglookout_settings** | Option<[**models::JsonSchemaPglookout**](json-schema-pglookout.md)> |  | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**version** | Option<**String**> | Version | [optional]
**plan** | Option<**String**> | Subscription plan | [optional]
**work_mem** | Option<**i64**> | Sets the maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files, in MB. Default is 1MB + 0.075% of total RAM (up to 32MB). | [optional]
**shared_buffers_percentage** | Option<**i64**> | Percentage of total RAM that the database server uses for shared memory buffers. Valid range is 20-60 (float), which corresponds to 20% - 60%. This setting adjusts the shared_buffers configuration value. | [optional]
**pg_settings** | Option<[**models::JsonSchemaPg**](json-schema-pg.md)> |  | [optional]
**migration** | Option<[**models::UpdateDbaasServiceMysqlRequestMigration**](update_dbaas_service_mysql_request_migration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


