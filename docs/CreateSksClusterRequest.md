# CreateSksClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**cni** | Option<**String**> | Cluster CNI | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**oidc** | Option<[**crate::models::SksOidc**](sks-oidc.md)> |  | [optional]
**name** | **String** | Cluster name | 
**level** | **String** | Cluster service level | 
**addons** | Option<**Vec<String>**> | Cluster addons | [optional]
**version** | **String** | Control plane Kubernetes version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


