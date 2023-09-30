# IamRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | IAM Role ID | [optional][readonly]
**name** | Option<**String**> | IAM Role name | [optional]
**description** | Option<**String**> | IAM Role description | [optional]
**permissions** | Option<**Vec<String>**> | IAM Role permissions | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**editable** | Option<**bool**> | IAM Role mutability | [optional]
**policy** | Option<[**crate::models::IamPolicy**](iam-policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


