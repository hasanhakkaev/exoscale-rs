# IngestMeteringRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flush_id** | **uuid::Uuid** | UUID identifying this flush; used for idempotent deduplication | 
**usage** | [**std::collections::HashMap<String, models::ApiKeyUsageEntry>**](ApiKeyUsageEntry.md) | Map of api-key-uuid to usage entry. Keys are API key UUIDs. Mirrors the router's in-memory accumulator structure directly. | 
**router_id** | **String** | Router instance identifier that produced this flush | 
**created_at** | **String** | ISO-8601 UTC timestamp when the flush snapshot was created (truncated to minute boundary for bucketing) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


