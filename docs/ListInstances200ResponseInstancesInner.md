# ListInstances200ResponseInstancesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Instance Security Groups | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | Option<[**models::InstanceType**](instance-type.md)> |  | [optional]
**private_networks** | Option<[**Vec<models::ListInstances200ResponseInstancesInnerPrivateNetworksInner>**](list_instances_200_response_instances_inner_private_networks_inner.md)> | Instance Private Networks | [optional]
**template** | Option<[**models::Template**](template.md)> |  | [optional]
**state** | Option<[**models::InstanceState**](instance-state.md)> |  | [optional]
**ssh_key** | Option<[**models::SshKey**](ssh-key.md)> |  | [optional]
**mac_address** | Option<**String**> | Instance MAC address | [optional]
**manager** | Option<[**models::Manager**](manager.md)> |  | [optional]
**ipv6_address** | Option<**String**> | Instance IPv6 address | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Instance ID | [optional]
**ssh_keys** | Option<[**Vec<models::SshKey>**](ssh-key.md)> | Instance SSH Keys | [optional]
**created_at** | Option<**String**> | Instance creation date | [optional]
**public_ip** | Option<**String**> | Instance public IPv4 address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


