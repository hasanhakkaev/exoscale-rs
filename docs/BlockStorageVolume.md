# BlockStorageVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**instance** | Option<[**models::InstanceTarget**](instance-target.md)> |  | [optional]
**name** | Option<**String**> | Volume name | [optional]
**state** | Option<**String**> | Volume state | [optional][readonly]
**size** | Option<**u64**> | Volume size | [optional]
**blocksize** | Option<**u64**> | Volume block size | [optional][readonly]
**block_storage_snapshots** | Option<[**Vec<models::BlockStorageSnapshotTarget>**](block-storage-snapshot-target.md)> | Volume snapshots, if any | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Volume ID | [optional][readonly]
**created_at** | Option<**String**> | Volume creation date | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


