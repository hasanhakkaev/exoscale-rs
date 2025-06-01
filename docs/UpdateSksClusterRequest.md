# UpdateSksClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Cluster name | [optional]
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**oidc** | Option<[**models::SksOidc**](sks-oidc.md)> |  | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**addons** | Option<**Vec<String>**> | Cluster addons | [optional]
**feature_gates** | Option<**Vec<String>**> | A list of Kubernetes-only Alpha features to enable for API server component | [optional]
**enable_operators_ca** | Option<**bool**> | Add or remove the operators certificate authority (CA) from the list of trusted CAs of the api server. The default value is true | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


