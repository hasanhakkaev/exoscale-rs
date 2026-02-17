# Operation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Operation ID | [optional][readonly]
**reason** | Option<**Reason**> | Operation failure reason (enum: incorrect, unknown, unavailable, forbidden, busy, fault, partial, not-found, interrupted, unsupported, conflict) | [optional][readonly]
**reference** | Option<[**models::OperationReference**](OperationReference.md)> |  | [optional]
**message** | Option<**String**> | Operation message | [optional][readonly]
**state** | Option<**State**> | Operation status (enum: failure, pending, success, timeout) | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


