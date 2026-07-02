# ReEncryptRequestSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **uuid::Uuid** | The ID of the source key currently protecting the data payload. | 
**encryption_context** | Option<**String**> | Optional Base64-encoded encryption context originally appended to the AAD to confirm package validation rules. | [optional]
**ciphertext** | **String** | The Base64-encoded encrypted payload package ready to undergo source-side key decryption. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


