# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | Operation unique identifier | [optional]
**zone** | Option<**String**> | Operation targeted zone | [optional]
**remote_addr** | Option<**String**> | Configuration IP address | [optional]
**get_params** | Option<[**serde_json::Value**](.md)> | Query string parameters (free form map) | [optional]
**body_params** | Option<[**serde_json::Value**](.md)> | Body parameters (free form map) | [optional]
**status** | Option<**i64**> | Operation HTTP status | [optional]
**uri** | Option<**String**> | Operation request URI | [optional]
**elapsed_ms** | Option<**i64**> | Operation processing time | [optional]
**timestamp** | Option<**String**> | Time at which the event happened, millisecond resolution | [optional]
**path_params** | Option<[**serde_json::Value**](.md)> | URI path parameters (free form map) | [optional]
**handler** | Option<**String**> | Operation handler name | [optional]
**message** | Option<**String**> | Operation message | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


