# DbaasMigrationStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | Option<**String**> | Error message in case that migration has failed | [optional]
**method** | Option<**String**> | Migration method. Empty in case of multiple methods or error | [optional]
**status** | Option<**String**> | Migration status | [optional]
**details** | Option<[**Vec<models::DbaasMigrationStatusDetailsInner>**](dbaas_migration_status_details_inner.md)> | Migration status per database | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


