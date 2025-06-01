# Snapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Snapshot ID | [optional][readonly]
**name** | Option<**String**> | Snapshot name | [optional]
**created_at** | Option<**String**> | Snapshot creation date | [optional][readonly]
**state** | Option<**String**> | Snapshot state | [optional]
**size** | Option<**u64**> | Snapshot size in GiB | [optional][readonly]
**export** | Option<[**models::SnapshotExport**](snapshot_export.md)> |  | [optional]
**instance** | Option<[**models::Instance**](instance.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


