# DbaasOpensearchAclConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acls** | Option<[**Vec<models::DbaasOpensearchAclConfigAclsInner>**](DbaasOpensearchAclConfigAclsInner.md)> | List of OpenSearch ACLs | [optional]
**acl_enabled** | Option<**bool**> | Enable OpenSearch ACLs. When disabled authenticated service users have unrestricted access. | [optional]
**extended_acl_enabled** | Option<**bool**> | Enable to enforce index rules in a limited fashion for requests that use the _mget, _msearch, and _bulk APIs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


