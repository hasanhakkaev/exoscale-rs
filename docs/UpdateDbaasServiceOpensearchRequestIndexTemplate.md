# UpdateDbaasServiceOpensearchRequestIndexTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mapping_nested_objects_limit** | Option<**i64**> | The maximum number of nested JSON objects that a single document can contain across all nested types. This limit helps to prevent out of memory errors when a document contains too many nested objects. Default is 10000. | [optional]
**number_of_replicas** | Option<**i64**> | The number of replicas each primary shard has. | [optional]
**number_of_shards** | Option<**i64**> | The number of primary shards that an index should have. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


