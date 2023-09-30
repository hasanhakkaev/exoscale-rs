# ListInstances200ResponseInstancesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_ip_assignment** | Option<[**crate::models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroup>**](security-group.md)> | Instance Security Groups | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**crate::models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<crate::models::PrivateNetwork>**](private-network.md)> | Instance Private Networks | [optional]
**template** | Option<[**crate::models::Template**](template.md)> |  | [optional]
**state** | Option<[**crate::models::InstanceState**](instance-state.md)> |  | [optional]
**ssh_key** | Option<[**crate::models::SshKey**](ssh-key.md)> |  | [optional]
**manager** | Option<[**crate::models::Manager**](manager.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance ID | [optional]
**ssh_keys** | Option<[**Vec<crate::models::SshKey>**](ssh-key.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


