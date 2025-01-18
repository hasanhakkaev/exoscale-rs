# CreateSksNodepoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroup>**](anti-affinity-group.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**public_ip_assignment** | Option<**String**> | Configures public IP assignment of the Instances with:  * IPv4 (`inet4`) addressing only (default); * both IPv4 and IPv6 (`dual`) addressing. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**std::collections::HashMap<String, models::SksNodepoolTaint>**](sks-nodepool-taint.md)> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Nodepool Security Groups | [optional]
**name** | **String** | Nodepool name, lowercase only | 
**instance_type** | [**models::InstanceType**](instance-type.md) |  | 
**private_networks** | Option<[**Vec<models::PrivateNetwork>**](private-network.md)> | Nodepool Private Networks | [optional]
**size** | **i64** | Number of instances | 
**kubelet_image_gc** | Option<[**models::KubeletImageGc**](kubelet-image-gc.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to instances names (default: pool), lowercase only | [optional]
**deploy_target** | Option<[**models::DeployTarget**](deploy-target.md)> |  | [optional]
**addons** | Option<**Vec<String>**> | Nodepool addons | [optional]
**disk_size** | **i64** | Nodepool instances disk size in GiB | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


