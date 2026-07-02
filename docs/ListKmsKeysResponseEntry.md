# ListKmsKeysResponseEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | An optional detailed description providing additional context about the key's intended use case. | [optional]
**rotation** | [**models::KeyRotationConfig**](KeyRotationConfig.md) |  | 
**revision** | [**models::RevisionStamp**](RevisionStamp.md) |  | 
**delete_at** | Option<**String**> |  | [optional]
**name** | **String** | The display name of the KMS key. | 
**multi_zone** | **bool** | True if this is a multi-zone key. | 
**source** | **Source** |  (enum: exoscale-kms) | 
**usage** | **String** | The cryptographic operation constraints allowed on this key. | 
**status** | **Status** |  (enum: enabled, disabled, pending-deletion) | 
**status_since** | **String** | The precise time when the key entered its current configuration phase. | 
**id** | **uuid::Uuid** | The globally unique identifier (UUID) tracking this key entity. | 
**replicas** | Option<**Vec<String>**> | Array tracking target zones currently maintaining copies of this item. | [optional]
**material** | [**models::KeyMaterial**](KeyMaterial.md) |  | 
**origin_zone** | **String** | The creation zone of the KMS key. | 
**created_at** | **String** | The UTC timestamp showing when the KMS key was originally provisioned. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


