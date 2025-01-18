# CreateIamRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | IAM Role name | 
**description** | Option<**String**> | IAM Role description | [optional]
**permissions** | Option<**Vec<String>**> | IAM Role permissions | [optional]
**editable** | Option<**bool**> | Sets if the IAM Role Policy is editable or not (default: true). This setting cannot be changed after creation | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**policy** | Option<[**models::IamPolicy**](iam-policy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


