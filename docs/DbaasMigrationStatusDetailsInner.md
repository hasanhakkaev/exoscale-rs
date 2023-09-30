# DbaasMigrationStatusDetailsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dbname** | Option<**String**> | Migrated db name (PG) or number (Redis) | [optional]
**error** | Option<**String**> | Error message in case that migration has failed | [optional]
**method** | Option<**String**> | Migration method | [optional]
**status** | Option<[**crate::models::EnumMigrationStatus**](enum-migration-status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


