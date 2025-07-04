# CreateDbaasServiceValkeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**plan** | **String** | Subscription plan | 
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**migration** | Option<[**models::UpdateDbaasServiceMysqlRequestMigration**](update_dbaas_service_mysql_request_migration.md)> |  | [optional]
**valkey_settings** | Option<[**models::JsonSchemaValkey**](json-schema-valkey.md)> |  | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**recovery_backup_name** | Option<**String**> | Name of a backup to recover from for services that support backup names | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


