# PrivateNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Private Network ID | [optional][readonly]
**name** | Option<**String**> | Private Network name | [optional]
**description** | Option<**String**> | Private Network description | [optional]
**netmask** | Option<**String**> | Private Network netmask | [optional]
**start_ip** | Option<**String**> | Private Network start IP address | [optional]
**end_ip** | Option<**String**> | Private Network end IP address | [optional]
**leases** | Option<[**Vec<crate::models::PrivateNetworkLease>**](private-network-lease.md)> | Private Network leased IP addresses | [optional][readonly]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


