# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anti_affinity_groups** | Option<[**Vec<models::AntiAffinityGroup>**](anti-affinity-group.md)> | Instance Anti-affinity Groups | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](public-ip-assignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**auto_start** | Option<**bool**> | Start Instance on creation (default: true) | [optional]
**security_groups** | Option<[**Vec<models::SecurityGroup>**](security-group.md)> | Instance Security Groups | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | [**models::InstanceType**](instance-type.md) |  | 
**template** | [**models::Template**](template.md) |  | 
**ssh_key** | Option<[**models::SshKey**](ssh-key.md)> |  | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**deploy_target** | Option<[**models::DeployTarget**](deploy-target.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6. DEPRECATED: use `public-ip-assignments`. | [optional]
**disk_size** | **i64** | Instance disk size in GiB | 
**ssh_keys** | Option<[**Vec<models::SshKey>**](ssh-key.md)> | Instance SSH Keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


