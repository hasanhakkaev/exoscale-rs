# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iam_user** | Option<[**models::User**](user.md)> |  | [optional]
**request_id** | Option<**String**> | Operation unique identifier | [optional]
**iam_role** | Option<[**models::IamRole**](iam-role.md)> |  | [optional]
**zone** | Option<**String**> | Operation targeted zone | [optional]
**get_params** | Option<[**serde_json::Value**](.md)> | Query string parameters (free form map) | [optional]
**body_params** | Option<[**serde_json::Value**](.md)> | Body parameters (free form map) | [optional]
**status** | Option<**u64**> | Operation HTTP status | [optional]
**source_ip** | Option<**String**> | Client IP address | [optional]
**iam_api_key** | Option<[**models::IamApiKey**](iam-api-key.md)> |  | [optional]
**uri** | Option<**String**> | Operation request URI | [optional]
**elapsed_ms** | Option<**u64**> | Operation processing time | [optional]
**timestamp** | Option<**String**> | Time at which the event happened, millisecond resolution | [optional]
**path_params** | Option<[**serde_json::Value**](.md)> | URI path parameters (free form map) | [optional]
**handler** | Option<**String**> | Operation handler name | [optional]
**message** | Option<**String**> | Operation message | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


