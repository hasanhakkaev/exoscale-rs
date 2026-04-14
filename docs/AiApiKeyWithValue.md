# AiApiKeyWithValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Last update timestamp | [optional][readonly]
**name** | Option<**String**> | Human-readable name for the AI API key | [optional]
**scope** | Option<**String**> | Key scope: 'public' for all deployments, or a specific deployment UUID | [optional]
**id** | Option<**uuid::Uuid**> | AI API key ID | [optional][readonly]
**org_uuid** | Option<**uuid::Uuid**> | Organization UUID that owns this key | [optional][readonly]
**created_at** | Option<**String**> | Creation timestamp | [optional][readonly]
**value** | **String** | Plaintext AI API key value (returned only on create/rotate) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


