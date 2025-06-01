# CreateSksClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**cni** | Option<**String**> | Cluster CNI | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**networking** | Option<[**models::Networking**](networking.md)> |  | [optional]
**oidc** | Option<[**models::SksOidc**](sks-oidc.md)> |  | [optional]
**name** | **String** | Cluster name | 
**enable_kube_proxy** | Option<**bool**> | Indicates whether to deploy the Kubernetes network proxy. When unspecified, defaults to `true` unless Cilium CNI is selected | [optional]
**level** | **String** | Cluster service level | 
**feature_gates** | Option<**Vec<String>**> | A list of Kubernetes-only Alpha features to enable for API server component | [optional]
**addons** | Option<**Vec<String>**> | Cluster addons | [optional]
**version** | **String** | Control plane Kubernetes version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


