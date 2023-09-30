# CreateDbaasTaskMigrationCheckRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_service_uri** | **String** | Service URI of the source MySQL or PostgreSQL database with admin credentials. | 
**method** | Option<[**crate::models::EnumMigrationMethod**](enum-migration-method.md)> |  | [optional]
**ignore_dbs** | Option<**String**> | Comma-separated list of databases, which should be ignored during migration (supported by MySQL only at the moment) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


