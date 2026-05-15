# ApiKeyUsageEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | Organization that owns this API key | 
**models** | [**std::collections::HashMap<String, models::ModelUsageCounters>**](ModelUsageCounters.md) | Map of model-uuid to accumulated counters. Keys are model UUIDs. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


