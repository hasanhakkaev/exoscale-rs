# ElasticIp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Elastic IP ID | [optional][readonly]
**ip** | Option<**String**> | Elastic IP address | [optional][readonly]
**addressfamily** | Option<**Addressfamily**> | Elastic IP address family (enum: inet4, inet6) | [optional][readonly]
**cidr** | Option<**String**> | Elastic IP cidr | [optional][readonly]
**description** | Option<**String**> | Elastic IP description | [optional]
**healthcheck** | Option<[**models::ElasticIpHealthcheck**](ElasticIpHealthcheck.md)> |  | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


