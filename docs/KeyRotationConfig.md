# KeyRotationConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**manual_count** | **i32** | Total running tally of manual key rotation tasks executed by users over this key resource's lifecycle. | 
**automatic** | **bool** | When set to true, dictates that the system automatically rotates material periodically. | 
**rotation_period** | **i32** | The set frequency period (measured in days) for triggers monitoring auto-rotation loops. | 
**next_at** | **String** | Scheduled deadline calculation pinpointing the next automated rotational iteration target date. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


