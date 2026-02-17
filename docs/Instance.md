# Instance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_consistent_snapshot_enabled** | Option<**bool**> | Indicates if the instance will take application-consistent snapshots | [optional]
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Instance Anti-affinity Groups | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](PublicIpAssignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Instance Security Groups | [optional]
**elastic_ips** | Option<[**Vec<models::ElasticIpRef>**](ElasticIpRef.md)> | Instance Elastic IPs | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**models::InstanceType**](InstanceType.md)> |  | [optional]
**private_networks** | Option<[**Vec<models::ListInstances200ResponseInstancesInnerPrivateNetworksInner>**](ListInstances200ResponseInstancesInnerPrivateNetworksInner.md)> | Instance Private Networks | [optional]
**template** | Option<[**models::Template**](Template.md)> |  | [optional]
**state** | Option<[**models::InstanceState**](InstanceState.md)> |  | [optional]
**secureboot_enabled** | Option<**bool**> | Indicates if the instance has secure boot enabled | [optional]
**ssh_key** | Option<[**models::SshKey**](SshKey.md)> |  | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**mac_address** | Option<**String**> | Instance MAC address | [optional][readonly]
**manager** | Option<[**models::Manager**](Manager.md)> |  | [optional]
**tpm_enabled** | Option<**bool**> | Indicates if the instance has tpm enabled | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional][readonly]
**id** | Option<**uuid::Uuid**> | Instance ID | [optional][readonly]
**snapshots** | Option<[**Vec<models::SnapshotRef>**](SnapshotRef.md)> | Instance Snapshots | [optional]
**disk_size** | Option<**u64**> | Instance disk size in GiB | [optional]
**ssh_keys** | Option<[**Vec<models::SshKey>**](SshKey.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional][readonly]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


