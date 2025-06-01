# CreateBlockStorageVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Volume name | [optional]
**size** | Option<**u64**> | Volume size in GiB.                             When a snapshot ID is supplied, this defaults to the size of the source volume, but can be set to a larger value. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**block_storage_snapshot** | Option<[**models::BlockStorageSnapshotTarget**](block-storage-snapshot-target.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


