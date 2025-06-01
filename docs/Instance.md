# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroup>**](anti-affinity-group.md)> | Instance Anti-affinity Groups | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Instance Security Groups | [optional]
**elastic_ips** | Option<[**Vec<models::ElasticIp>**](elastic-ip.md)> | Instance Elastic IPs | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<models::ListInstances200ResponseInstancesInnerPrivateNetworksInner>**](list_instances_200_response_instances_inner_private_networks_inner.md)> | Instance Private Networks | [optional]
**template** | Option<[**models::Template**](template.md)> |  | [optional]
**state** | Option<[**models::InstanceState**](instance-state.md)> |  | [optional]
**secureboot_enabled** | Option<**bool**> | Indicates if the instance has secure boot enabled | [optional]
**ssh_key** | Option<[**models::SshKey**](ssh-key.md)> |  | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**mac_address** | Option<**String**> | Instance MAC address | [optional][readonly]
**manager** | Option<[**models::Manager**](manager.md)> |  | [optional]
**tpm_enabled** | Option<**bool**> | Indicates if the instance has tpm enabled | [optional]
**deploy_target** | Option<[**models::DeployTarget**](deploy-target.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional][readonly]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance ID | [optional][readonly]
**snapshots** | Option<[**Vec<models::Snapshot>**](snapshot.md)> | Instance Snapshots | [optional]
**disk_size** | Option<**u64**> | Instance disk size in GiB | [optional]
**ssh_keys** | Option<[**Vec<models::SshKey>**](ssh-key.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional][readonly]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


