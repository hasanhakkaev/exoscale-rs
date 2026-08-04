# Subnet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Subnet description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**name** | Option<**String**> | Subnet name | [optional]
**ipv4_block** | Option<**String**> | Subnet ipv4 CIDR | [optional]
**addressfamily** | Option<**Addressfamily**> | Subnet address family (enum: inet4, dual) | [optional]
**address_space** | Option<**AddressSpace**> | Subnet address space (enum: private) | [optional]
**instances** | Option<[**Vec<models::SubnetInstancesInner>**](SubnetInstancesInner.md)> | Instances attached to the subnet | [optional]
**id** | Option<**uuid::Uuid**> | Subnet ID | [optional]
**created_at** | Option<**String**> | Subnet creation date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


