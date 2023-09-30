# UpdateInstancePoolRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<crate::models::AntiAffinityGroup>**](anti-affinity-group.md)> | Instance Pool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Instance Pool description | [optional]
**public_ip_assignment** | Option<[**crate::models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroup>**](security-group.md)> | Instance Pool Security Groups | [optional]
**elastic_ips** | Option<[**Vec<crate::models::ElasticIp>**](elastic-ip.md)> | Instances Elastic IPs | [optional]
**name** | Option<**String**> | Instance Pool name | [optional]
**instance_type** | Option<[**crate::models::InstanceType**](instance-type.md)> |  | [optional]
**min_available** | Option<**i64**> | Minimum number of running Instances | [optional]
**private_networks** | Option<[**Vec<crate::models::PrivateNetwork>**](private-network.md)> | Instance Pool Private Networks | [optional]
**template** | Option<[**crate::models::Template**](template.md)> |  | [optional]
**ssh_key** | Option<[**crate::models::SshKey**](ssh-key.md)> |  | [optional]
**instance_prefix** | Option<**String**> | Prefix to apply to Instances names (default: pool) | [optional]
**user_data** | Option<**String**> | Instances Cloud-init user-data | [optional]
**deploy_target** | Option<[**crate::models::DeployTarget**](deploy-target.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6. DEPRECATED: use `public-ip-assignments`. | [optional]
**disk_size** | Option<**i64**> | Instances disk size in GB | [optional]
**ssh_keys** | Option<[**Vec<crate::models::SshKey>**](ssh-key.md)> | Instances SSH keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


