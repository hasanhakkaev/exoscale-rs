# UpdateInstanceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Instance name | [optional]
**user_data** | Option<**String**> | Instance Cloud-init user-data (base64 encoded) | [optional]
**public_ip_assignment** | Option<[**models::PublicIpAssignment**](PublicIpAssignment.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**application_consistent_snapshot_enabled** | Option<**bool**> | Enable/Disable Application Consistent Snapshot for Instance | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


