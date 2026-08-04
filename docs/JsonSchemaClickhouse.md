# JsonSchemaClickhouse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server_settings** | Option<[**models::ClickHouseServerSettings**](ClickHouseServerSettings.md)> |  | [optional]
**tiered_storage_move_factor** | Option<**f64**> | The percentage of free disk space required on local storage before data is moved to object storage. A value of 0.2 means data is moved when local storage has less than 20% free space. | [optional][default to 0.2]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


