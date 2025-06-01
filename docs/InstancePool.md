# InstancePool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroup>**](anti-affinity-group.md)> | Instance Pool Anti-affinity Groups | [optional]
**description** | Option<**String**> | Instance Pool description | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Instance Pool Security Groups | [optional]
**elastic_ips** | Option<[**Vec<models::ElasticIp>**](elastic-ip.md)> | Instances Elastic IPs | [optional]
**name** | Option<**String**> | Instance Pool name | [optional]
**instance_type** | Option<[**models::InstanceType**](instance-type.md)> |  | [optional]
**min_available** | Option<**u64**> | Minimum number of running instances | [optional]
**private_networks** | Option<[**Vec<models::PrivateNetwork>**](private-network.md)> | Instance Pool Private Networks | [optional]
**template** | Option<[**models::Template**](template.md)> |  | [optional]
**state** | Option<**String**> | Instance Pool state | [optional][readonly]
**size** | Option<**u64**> | Number of instances | [optional]
**ssh_key** | Option<[**models::SshKey**](ssh-key.md)> |  | [optional]
**instance_prefix** | Option<**String**> | The instances created by the Instance Pool will be prefixed with this value (default: pool) | [optional]
**user_data** | Option<**String**> | Instances Cloud-init user-data | [optional]
**manager** | Option<[**models::Manager**](manager.md)> |  | [optional]
**instances** | Option<[**Vec<models::Instance>**](instance.md)> | Instances | [optional][readonly]
**deploy_target** | Option<[**models::DeployTarget**](deploy-target.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6 for instances | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance Pool ID | [optional][readonly]
**disk_size** | Option<**u64**> | Instances disk size in GiB | [optional]
**ssh_keys** | Option<[**Vec<models::SshKey>**](ssh-key.md)> | Instances SSH keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


