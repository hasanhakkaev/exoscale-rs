# SksNodepool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroup>**](anti-affinity-group.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**public_ip_assignment** | Option<**String**> | Nodepool public IP assignment of the Instances:  * IPv4 (`inet4`) addressing only; * IPv4 and IPv6 (`dual`) addressing. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**std::collections::HashMap<String, models::SksNodepoolTaint>**](sks-nodepool-taint.md)> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Nodepool Security Groups | [optional]
**name** | Option<**String**> | Nodepool name | [optional]
**instance_type** | Option<[**models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<models::PrivateNetwork>**](private-network.md)> | Nodepool Private Networks | [optional]
**template** | Option<[**models::Template**](template.md)> |  | [optional]
**state** | Option<**String**> | Nodepool state | [optional][readonly]
**size** | Option<**u64**> | Number of instances | [optional]
**kubelet_image_gc** | Option<[**models::KubeletImageGc**](kubelet-image-gc.md)> |  | [optional]
**instance_pool** | Option<[**models::InstancePool**](instance-pool.md)> |  | [optional]
**instance_prefix** | Option<**String**> | The instances created by the Nodepool will be prefixed with this value (default: pool) | [optional]
**deploy_target** | Option<[**models::DeployTarget**](deploy-target.md)> |  | [optional]
**addons** | Option<**Vec<String>**> | Nodepool addons | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Nodepool ID | [optional][readonly]
**disk_size** | Option<**u64**> | Nodepool instances disk size in GiB | [optional]
**version** | Option<**String**> | Nodepool version | [optional][readonly]
**created_at** | Option<**String**> | Nodepool creation date | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


