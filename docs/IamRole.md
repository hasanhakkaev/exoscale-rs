# IamRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | IAM Role description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**permissions** | Option<**HashSet<Permissions>**> | IAM Role permissions (enum: bypass-governance-retention, reset-iam-organization-policy) | [optional]
**assume_role_policy** | Option<[**models::IamPolicy**](IamPolicy.md)> |  | [optional]
**editable** | Option<**bool**> | IAM Role mutability | [optional]
**name** | Option<**String**> | IAM Role name | [optional]
**max_session_ttl** | Option<**u64**> | Maximum TTL requester is allowed to ask for when assuming a role | [optional]
**policy** | Option<[**models::IamPolicy**](IamPolicy.md)> |  | [optional]
**id** | Option<**uuid::Uuid**> | IAM Role ID | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


