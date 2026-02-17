# CreateSksClusterRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Cluster description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**cni** | Option<**Cni**> | Cluster CNI (enum: calico, cilium) | [optional]
**auto_upgrade** | Option<**bool**> | Enable auto upgrade of the control plane to the latest patch version available | [optional]
**networking** | Option<[**models::Networking**](Networking.md)> |  | [optional]
**oidc** | Option<[**models::SksOidc**](SksOidc.md)> |  | [optional]
**name** | **String** | Cluster name | 
**create_default_security_group** | Option<**bool**> | Creates an ad-hoc security group based on the choice of the selected CNI | [optional]
**enable_kube_proxy** | Option<**bool**> | Indicates whether to deploy the Kubernetes network proxy. When unspecified, defaults to `true` unless Cilium CNI is selected | [optional]
**level** | **Level** | Cluster service level (enum: starter, pro) | 
**feature_gates** | Option<**HashSet<String>**> | A list of Kubernetes-only Alpha features to enable for API server component | [optional]
**addons** | Option<**HashSet<Addons>**> | Cluster addons (enum: exoscale-cloud-controller, exoscale-container-storage-interface, metrics-server, karpenter) | [optional]
**audit** | Option<[**models::SksAuditCreate**](SksAuditCreate.md)> |  | [optional]
**version** | **String** | Control plane Kubernetes version | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


