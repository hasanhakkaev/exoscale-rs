# Snapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Snapshot ID | [optional][readonly]
**name** | Option<**String**> | Snapshot name | [optional]
**created_at** | Option<**String**> | Snapshot creation date | [optional][readonly]
**state** | Option<**State**> | Snapshot state (enum: snapshotting, deleted, exporting, ready, deleting, error, exported) | [optional]
**size** | Option<**u64**> | Snapshot size in GiB | [optional][readonly]
**export** | Option<[**models::SnapshotExport**](SnapshotExport.md)> |  | [optional]
**instance** | Option<[**models::Instance**](Instance.md)> |  | [optional]
**application_consistent** | Option<**bool**> | Indicates whether the snapshot was taken using an application-consistent method | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


