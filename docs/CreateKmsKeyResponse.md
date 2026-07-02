# CreateKmsKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | An optional detailed description providing additional context about the key's intended use case. | [optional]
**revision** | [**models::RevisionStamp**](RevisionStamp.md) |  | 
**name** | **String** | The display name assigned to the KMS key. | 
**multi_zone** | **bool** | True if this is a multi-zone key. | 
**source** | **Source** |  (enum: exoscale-kms) | 
**usage** | **String** | The cryptographic operation constraints allowed on this key. | 
**status** | **Status** |  (enum: enabled, disabled, pending-deletion) | 
**status_since** | **String** | The timestamp indicating exactly when the current key status was last transitioned. | 
**id** | **uuid::Uuid** | The globally unique identifier (UUID) assigned to the newly created KMS key. | 
**origin_zone** | **String** | The creation zone of the KMS key. | 
**created_at** | **String** | The UTC timestamp showing when the KMS key was originally provisioned. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


