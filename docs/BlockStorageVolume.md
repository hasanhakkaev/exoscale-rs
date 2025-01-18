# BlockStorageVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Volume ID | [optional][readonly]
**name** | Option<**String**> | Volume name | [optional]
**created_at** | Option<**String**> | Volume creation date | [optional][readonly]
**state** | Option<**String**> | Volume state | [optional][readonly]
**size** | Option<**i64**> | Volume size | [optional]
**blocksize** | Option<**i64**> | Volume block size | [optional][readonly]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**instance** | Option<[**models::InstanceTarget**](instance-target.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


