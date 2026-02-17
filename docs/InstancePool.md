# InstancePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_consistent_snapshot_enabled** | Option<**bool**> | Enable application consistent snapshots | [optional]
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Instance Pool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Instance Pool description | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](PublicIpAssignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Instance Pool Security Groups | [optional]
**elastic_ips** | Option<[**Vec<models::ElasticIpRef>**](ElasticIpRef.md)> | Instances Elastic IPs | [optional]
**name** | Option<**String**> | Instance Pool name | [optional]
**instance_type** | Option<[**models::InstanceTypeRef**](InstanceTypeRef.md)> |  | [optional]
**min_available** | Option<**u64**> | Minimum number of running instances | [optional]
**private_networks** | Option<[**Vec<models::PrivateNetworkRef>**](PrivateNetworkRef.md)> | Instance Pool Private Networks | [optional]
**template** | Option<[**models::TemplateRef**](TemplateRef.md)> |  | [optional]
**state** | Option<**State**> | Instance Pool state (enum: scaling-up, scaling-down, destroying, creating, suspended, running, updating) | [optional][readonly]
**size** | Option<**u64**> | Number of instances | [optional]
**ssh_key** | Option<[**models::SshKeyRef**](SshKeyRef.md)> |  | [optional]
**instance_prefix** | Option<**String**> | The instances created by the Instance Pool will be prefixed with this value (default: pool) | [optional]
**user_data** | Option<**String**> | Instances Cloud-init user-data | [optional]
**manager** | Option<[**models::Manager**](Manager.md)> |  | [optional]
**instances** | Option<[**Vec<models::InstanceRef>**](InstanceRef.md)> | Instances | [optional][readonly]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6 for instances | [optional]
**id** | Option<**uuid::Uuid**> | Instance Pool ID | [optional][readonly]
**disk_size** | Option<**u64**> | Instances disk size in GiB | [optional]
**ssh_keys** | Option<[**Vec<models::SshKeyRef>**](SshKeyRef.md)> | Instances SSH keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


