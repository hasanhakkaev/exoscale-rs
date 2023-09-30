# SecondaryVolumeSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Volume ID | [optional][readonly]
**name** | Option<**String**> | Volume snapshot name | [optional]
**size** | Option<**i64**> | Volume size | [optional]
**created_at** | Option<**String**> | Volume snapshot creation date | [optional][readonly]
**state** | Option<**String**> | Volume snapshot state | [optional][readonly]
**secondary_volume** | Option<[**crate::models::SecondaryVolumeTarget**](secondary-volume-target.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


