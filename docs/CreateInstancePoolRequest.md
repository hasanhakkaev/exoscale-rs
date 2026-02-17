# CreateInstancePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_consistent_snapshot_enabled** | Option<**bool**> | Enable application consistent snapshots | [optional]
**anti_affinity_groups** | Option<[**HashSet<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Instance Pool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Instance Pool description | [optional]
**public_ip_assignment** | Option<**PublicIpAssignment**> | Determines public IP assignment of the Instances. Type `none` is final and can't be changed later on. (enum: inet4, dual, none) | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**HashSet<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Instance Pool Security Groups | [optional]
**elastic_ips** | Option<[**HashSet<models::ElasticIpRef>**](ElasticIpRef.md)> | Instances Elastic IPs | [optional]
**name** | **String** | Instance Pool name | 
**instance_type** | [**models::InstanceTypeRef**](InstanceTypeRef.md) |  | 
**min_available** | Option<**u64**> | Minimum number of running Instances | [optional]
**private_networks** | Option<[**HashSet<models::PrivateNetworkRef>**](PrivateNetworkRef.md)> | Instance Pool Private Networks | [optional]
**template** | [**models::TemplateRef**](TemplateRef.md) |  | 
**size** | **u64** | Number of Instances | 
**ssh_key** | Option<[**models::SshKeyRef**](SshKeyRef.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to Instances names (default: pool) | [optional]
**user_data** | Option<**String**> | Instances Cloud-init user-data | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6. DEPRECATED: use `public-ip-assignments`. | [optional]
**disk_size** | **u64** | Instances disk size in GiB | 
**ssh_keys** | Option<[**HashSet<models::SshKeyRef>**](SshKeyRef.md)> | Instances SSH Keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


