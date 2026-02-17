# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sso** | Option<**bool**> | SSO enabled | [optional]
**two_factor_authentication** | Option<**bool**> | Two Factor Authentication enabled | [optional]
**email** | **String** | User Email | [readonly]
**id** | Option<**uuid::Uuid**> | User ID | [optional][readonly]
**role** | [**models::IamRole**](IamRole.md) |  | 
**pending** | Option<**bool**> | True if the user has not yet created an Exoscale account | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


