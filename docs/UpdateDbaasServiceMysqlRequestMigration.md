# UpdateDbaasServiceMysqlRequestMigration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** | Hostname or IP address of the server where to migrate data from | 
**port** | **i64** | Port number of the server where to migrate data from | 
**password** | Option<**String**> | Password for authentication with the server where to migrate data from | [optional]
**ssl** | Option<**bool**> | The server where to migrate data from is secured with SSL | [optional]
**username** | Option<**String**> | User name for authentication with the server where to migrate data from | [optional]
**dbname** | Option<**String**> | Database name for bootstrapping the initial connection | [optional]
**ignore_dbs** | Option<**String**> | Comma-separated list of databases, which should be ignored during migration (supported by MySQL only at the moment) | [optional]
**method** | Option<[**crate::models::EnumMigrationMethod**](enum-migration-method.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


