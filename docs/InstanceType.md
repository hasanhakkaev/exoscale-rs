# InstanceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Instance type ID | [optional][readonly]
**size** | Option<**Size**> | Instance type size (enum: large, huge, jumbo, medium, mega, small, extra-large, titan48c, titan, micro, colossus, tiny) | [optional][readonly]
**family** | Option<**Family**> | Instance type family (enum: gpu3, gpua30, gpu3080ti, gpu2, gpu, memory, gpua5000, gpurtx6000pro, storage, standard, colossus, cpu) | [optional][readonly]
**cpus** | Option<**u64**> | CPU count | [optional][readonly]
**gpus** | Option<**u64**> | GPU count | [optional][readonly]
**authorized** | Option<**bool**> | Requires authorization or publicly available | [optional][readonly]
**memory** | Option<**u64**> | Available memory | [optional][readonly]
**zones** | Option<[**Vec<models::ZoneName>**](ZoneName.md)> | Instance Type available zones | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


