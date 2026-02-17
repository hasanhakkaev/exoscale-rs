# ListInstances200ResponseInstancesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](PublicIpAssignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](SecurityGroup.md)> | Instance Security Groups | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**models::InstanceType**](InstanceType.md)> |  | [optional]
**private_networks** | Option<[**Vec<models::ListInstances200ResponseInstancesInnerPrivateNetworksInner>**](ListInstances200ResponseInstancesInnerPrivateNetworksInner.md)> | Instance Private Networks | [optional]
**template** | Option<[**models::Template**](Template.md)> |  | [optional]
**state** | Option<[**models::InstanceState**](InstanceState.md)> |  | [optional]
**ssh_key** | Option<[**models::SshKey**](SshKey.md)> |  | [optional]
**mac_address** | Option<**String**> | Instance MAC address | [optional]
**manager** | Option<[**models::Manager**](Manager.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional]
**id** | Option<**uuid::Uuid**> | Instance ID | [optional]
**ssh_keys** | Option<[**Vec<models::SshKey>**](SshKey.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


