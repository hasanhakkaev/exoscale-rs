# UpdateIamRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | IAM Role description | [optional]
**permissions** | Option<**HashSet<Permissions>**> | IAM Role permissions (enum: bypass-governance-retention, reset-iam-organization-policy) | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**max_session_ttl** | Option<**u64**> | Maximum TTL requester is allowed to ask for when assuming a role | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


