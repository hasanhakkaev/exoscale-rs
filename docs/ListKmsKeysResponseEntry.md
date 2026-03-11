# ListKmsKeysResponseEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** |  | 
**rotation** | [**models::KeyRotationConfig**](KeyRotationConfig.md) |  | 
**revision** | [**models::RevisionStamp**](RevisionStamp.md) |  | 
**name** | **String** |  | 
**multi_zone** | **bool** |  | 
**source** | **Source** |  (enum: exoscale-kms) | 
**usage** | **String** |  | 
**status** | **Status** |  (enum: enabled, disabled, pending-deletion) | 
**status_since** | **String** |  | 
**id** | **uuid::Uuid** |  | 
**replicas** | **Vec<String>** |  | 
**material** | [**models::KeyMaterial**](KeyMaterial.md) |  | 
**origin_zone** | **String** |  | 
**created_at** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


