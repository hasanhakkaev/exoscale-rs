# CreateDbaasServiceValkeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valkey_settings** | Option<[**models::JsonSchemaValkey**](JsonSchemaValkey.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](UpdateDbaasServiceMysqlRequestMaintenance.md)> |  | [optional]
**version** | Option<**String**> | Valkey major version | [optional]
**recovery_backup_name** | Option<**String**> | Name of a backup to recover from for services that support backup names | [optional]
**plan** | **String** | Subscription plan | 
**migration** | Option<[**models::UpdateDbaasServiceMysqlRequestMigration**](UpdateDbaasServiceMysqlRequestMigration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


