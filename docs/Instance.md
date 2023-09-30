# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<crate::models::AntiAffinityGroup>**](anti-affinity-group.md)> | Instance Anti-affinity Groups | [optional]
**public_ip_assignment** | Option<[**crate::models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroup>**](security-group.md)> | Instance Security Groups | [optional]
**elastic_ips** | Option<[**Vec<crate::models::ElasticIp>**](elastic-ip.md)> | Instance Elastic IPs | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**crate::models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<crate::models::PrivateNetwork>**](private-network.md)> | Instance Private Networks | [optional]
**template** | Option<[**crate::models::Template**](template.md)> |  | [optional]
**state** | Option<[**crate::models::InstanceState**](instance-state.md)> |  | [optional]
**ssh_key** | Option<[**crate::models::SshKey**](ssh-key.md)> |  | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**manager** | Option<[**crate::models::Manager**](manager.md)> |  | [optional]
**deploy_target** | Option<[**crate::models::DeployTarget**](deploy-target.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional][readonly]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance ID | [optional][readonly]
**snapshots** | Option<[**Vec<crate::models::Snapshot>**](snapshot.md)> | Instance Snapshots | [optional]
**disk_size** | Option<**i64**> | Instance disk size in GB | [optional]
**ssh_keys** | Option<[**Vec<crate::models::SshKey>**](ssh-key.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional][readonly]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


