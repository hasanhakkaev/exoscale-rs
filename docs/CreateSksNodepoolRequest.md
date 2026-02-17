# CreateSksNodepoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**HashSet<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**public_ip_assignment** | Option<**PublicIpAssignment**> | Configures public IP assignment of the Instances with:  * IPv4 (`inet4`) addressing only (default); * both IPv4 and IPv6 (`dual`) addressing. (enum: inet4, dual) | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**std::collections::HashMap<String, models::SksNodepoolTaint>**](SksNodepoolTaint.md)> |  | [optional]
**security_groups** | Option<[**HashSet<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Nodepool Security Groups | [optional]
**name** | **String** | Nodepool name, lowercase only | 
**instance_type** | [**models::InstanceTypeRef**](InstanceTypeRef.md) |  | 
**private_networks** | Option<[**HashSet<models::PrivateNetworkRef>**](PrivateNetworkRef.md)> | Nodepool Private Networks | [optional]
**size** | **u64** | Number of instances | 
**kubelet_image_gc** | Option<[**models::KubeletImageGc**](KubeletImageGc.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to instances names (default: pool), lowercase only | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**addons** | Option<**HashSet<Addons>**> | Nodepool addons (enum: storage-lvm) | [optional]
**disk_size** | **u64** | Nodepool instances disk size in GiB | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


