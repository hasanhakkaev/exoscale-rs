# PrivateNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Private Network description | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**name** | Option<**String**> | Private Network name | [optional]
**start_ip** | Option<**String**> | Private Network start IP address | [optional]
**leases** | Option<[**Vec<models::PrivateNetworkLease>**](private-network-lease.md)> | Private Network leased IP addresses | [optional][readonly]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Private Network ID | [optional][readonly]
**vni** | Option<**i64**> | Private Network VXLAN ID | [optional]
**netmask** | Option<**String**> | Private Network netmask | [optional]
**options** | Option<[**models::PrivateNetworkOptions**](private-network-options.md)> |  | [optional]
**end_ip** | Option<**String**> | Private Network end IP address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


