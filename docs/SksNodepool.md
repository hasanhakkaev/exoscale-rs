# SksNodepool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<crate::models::AntiAffinityGroup>**](anti-affinity-group.md)> | Nodepool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Nodepool description | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**taints** | Option<[**::std::collections::HashMap<String, crate::models::SksNodepoolTaint>**](sks-nodepool-taint.md)> |  | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroup>**](security-group.md)> | Nodepool Security Groups | [optional]
**name** | Option<**String**> | Nodepool name | [optional]
**instance_type** | Option<[**crate::models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<crate::models::PrivateNetwork>**](private-network.md)> | Nodepool Private Networks | [optional]
**template** | Option<[**crate::models::Template**](template.md)> |  | [optional]
**state** | Option<**String**> | Nodepool state | [optional][readonly]
**size** | Option<**i64**> | Number of instances | [optional]
**kubelet_image_gc** | Option<[**crate::models::KubeletImageGc**](kubelet-image-gc.md)> |  | [optional]
**instance_pool** | Option<[**crate::models::InstancePool**](instance-pool.md)> |  | [optional]
**instance_prefix** | Option<**String**> | The instances created by the Nodepool will be prefixed with this value (default: pool) | [optional]
**deploy_target** | Option<[**crate::models::DeployTarget**](deploy-target.md)> |  | [optional]
**addons** | Option<**Vec<String>**> | Nodepool addons | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Nodepool ID | [optional][readonly]
**disk_size** | Option<**i64**> | Nodepool instances disk size in GB | [optional]
**version** | Option<**String**> | Nodepool version | [optional][readonly]
**created_at** | Option<**String**> | Nodepool creation date | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


