# InstanceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance type ID | [optional][readonly]
**size** | Option<**String**> | Instance type size | [optional][readonly]
**family** | Option<**String**> | Instance type family | [optional][readonly]
**cpus** | Option<**u64**> | CPU count | [optional][readonly]
**gpus** | Option<**u64**> | GPU count | [optional][readonly]
**authorized** | Option<**bool**> | Requires authorization or publicly available | [optional][readonly]
**memory** | Option<**u64**> | Available memory | [optional][readonly]
**zones** | Option<[**Vec<models::ZoneName>**](zone-name.md)> | Instance Type available zones | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


