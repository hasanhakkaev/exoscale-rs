# BlockStorageSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Snapshot ID | [optional][readonly]
**name** | Option<**String**> | Snapshot name | [optional]
**size** | Option<**i64**> | Snapshot size | [optional]
**volume_size** | Option<**i64**> | Original Volume size | [optional]
**created_at** | Option<**String**> | Snapshot creation date | [optional][readonly]
**state** | Option<**String**> | Snapshot state | [optional][readonly]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**block_storage_volume** | Option<[**models::BlockStorageVolumeTarget**](block-storage-volume-target.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


