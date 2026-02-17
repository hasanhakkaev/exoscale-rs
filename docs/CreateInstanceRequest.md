# CreateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**application_consistent_snapshot_enabled** | Option<**bool**> | Enable application-consistent snapshot for the instance | [optional]
**anti_affinity_groups** | Option<[**HashSet<models::AntiAffinityGroupRef>**](AntiAffinityGroupRef.md)> | Instance Anti-affinity Groups | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](PublicIpAssignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**auto_start** | Option<**bool**> | Start Instance on creation (default: true) | [optional]
**security_groups** | Option<[**HashSet<models::SecurityGroupRef>**](SecurityGroupRef.md)> | Instance Security Groups | [optional]
**name** | Option<**String**> | Instance name | [optional]
**instance_type** | [**models::InstanceTypeRef**](InstanceTypeRef.md) |  | 
**template** | [**models::TemplateRef**](TemplateRef.md) |  | 
**secureboot_enabled** | Option<**bool**> | Enable secure boot | [optional]
**ssh_key** | Option<[**models::SshKeyRef**](SshKeyRef.md)> |  | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**tpm_enabled** | Option<**bool**> | Enable Trusted Platform Module (TPM) | [optional]
**deploy_target** | Option<[**models::DeployTargetRef**](DeployTargetRef.md)> |  | [optional]
**ipv6_enabled** | Option<**bool**> | Enable IPv6. DEPRECATED: use `public-ip-assignments`. | [optional]
**disk_size** | **u64** | Instance disk size in GiB | 
**ssh_keys** | Option<[**HashSet<models::SshKeyRef>**](SshKeyRef.md)> | Instance SSH Keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


