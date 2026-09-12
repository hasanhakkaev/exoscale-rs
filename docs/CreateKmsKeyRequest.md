# CreateKmsKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | A human-readable display name uniquely identifying the KMS key within the tenant space. | 
**description** | Option<**String**> | An optional detailed description providing additional context about the key's intended use case. | [optional]
**usage** | Option<**Usage**> |  (enum: encrypt-decrypt) | [optional][default to EncryptDecrypt]
**multi_zone** | Option<**bool**> | True if this is a multi-zone key. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


