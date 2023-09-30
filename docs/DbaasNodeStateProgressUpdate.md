# DbaasNodeStateProgressUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed** | **bool** | Indicates whether this phase has been completed or not | 
**current** | Option<**i64**> | Current progress for this phase. May be missing or null. | [optional]
**max** | Option<**i64**> | Maximum progress value for this phase. May be missing or null. May change. | [optional]
**min** | Option<**i64**> | Minimum progress value for this phase. May be missing or null. | [optional]
**phase** | **String** | Key identifying this phase | 
**unit** | Option<**String**> | Unit for current/min/max values. New units may be added.                         If null should be treated as generic unit | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


