# UpdateSksNodepoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**HashSet<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**public_ip_assignment** | Option<**PublicIpAssignment**> | Configures public IP assignment of the Instances with:  * IPv4 (`inet4`) addressing only; * both IPv4 and IPv6 (`dual`) addressing. (enum: inet4, dual) | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**std::collections::HashMap<String, models::SksNodepoolTaint>**](SksNodepoolTaint.md)> |  | [optional]
**security_groups** | Option<[**HashSet<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Nodepool Security Groups | [optional]
**name** | Option<**String**> | Nodepool name, lowercase only | [optional]
**instance_type** | Option<[**models::InstanceTypeRef**](InstanceTypeRef.md)> |  | [optional]
**private_networks** | Option<[**HashSet<models::PrivateNetworkRef>**](PrivateNetworkRef.md)> | Nodepool Private Networks | [optional]
**kubelet_image_gc** | Option<[**models::KubeletImageGc**](KubeletImageGc.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to managed instances names (default: pool), lowercase only | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**disk_size** | Option<**u64**> | Nodepool instances disk size in GiB | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


