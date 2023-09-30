# SksOidc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | OpenID client ID | 
**issuer_url** | **String** | OpenID provider URL | 
**username_claim** | Option<**String**> | JWT claim to use as the user name | [optional]
**username_prefix** | Option<**String**> | Prefix prepended to username claims | [optional]
**groups_claim** | Option<**String**> | JWT claim to use as the user's group | [optional]
**groups_prefix** | Option<**String**> | Prefix prepended to group claims | [optional]
**required_claim** | Option<**::std::collections::HashMap<String, String>**> | A key value map that describes a required claim in the ID Token | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


