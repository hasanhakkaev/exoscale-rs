# CreateSksNodepoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<crate::models::AntiAffinityGroup>**](anti-affinity-group.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**::std::collections::HashMap<String, crate::models::SksNodepoolTaint>**](sks-nodepool-taint.md)> |  | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroup>**](security-group.md)> | Nodepool Security Groups | [optional]
**name** | **String** | Nodepool name | 
**instance_type** | [**crate::models::InstanceType**](instance-type.md) |  | 
**private_networks** | Option<[**Vec<crate::models::PrivateNetwork>**](private-network.md)> | Nodepool Private Networks | [optional]
**size** | **i64** | Number of instances | 
**kubelet_image_gc** | Option<[**crate::models::KubeletImageGc**](kubelet-image-gc.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to instances names (default: pool) | [optional]
**deploy_target** | Option<[**crate::models::DeployTarget**](deploy-target.md)> |  | [optional]
**addons** | Option<**Vec<String>**> | Nodepool addons | [optional]
**disk_size** | **i64** | Nodepool instances disk size in GB | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


