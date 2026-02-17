# SksNodepool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**HashSet<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**public_ip_assignment** | Option<**PublicIpAssignment**> | Nodepool public IP assignment of the Instances:  * IPv4 (`inet4`) addressing only; * IPv4 and IPv6 (`dual`) addressing. (enum: inet4, dual) | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**std::collections::HashMap<String, models::SksNodepoolTaint>**](SksNodepoolTaint.md)> |  | [optional]
**security_groups** | Option<[**HashSet<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Nodepool Security Groups | [optional]
**name** | Option<**String**> | Nodepool name | [optional]
**instance_type** | Option<[**models::InstanceTypeRef**](InstanceTypeRef.md)> |  | [optional]
**private_networks** | Option<[**HashSet<models::PrivateNetworkRef>**](PrivateNetworkRef.md)> | Nodepool Private Networks | [optional]
**template** | Option<[**models::TemplateRef**](TemplateRef.md)> |  | [optional]
**state** | Option<**State**> | Nodepool state (enum: renewing-token, creating, deleting, running, scaling, updating, error) | [optional][readonly]
**size** | Option<**u64**> | Number of instances | [optional]
**kubelet_image_gc** | Option<[**models::KubeletImageGc**](KubeletImageGc.md)> |  | [optional]
**instance_pool** | Option<[**models::InstancePoolRef**](InstancePoolRef.md)> |  | [optional]
**instance_prefix** | Option<**String**> | The instances created by the Nodepool will be prefixed with this value (default: pool) | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**addons** | Option<**HashSet<Addons>**> | Nodepool addons (enum: storage-lvm) | [optional]
**id** | Option<**uuid::Uuid**> | Nodepool ID | [optional][readonly]
**disk_size** | Option<**u64**> | Nodepool instances disk size in GiB | [optional]
**version** | Option<**String**> | Nodepool version | [optional][readonly]
**created_at** | Option<**String**> | Nodepool creation date | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


