# DbaasServiceComponents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**component** | **String** | Service component name | 
**host** | **String** | DNS name for connecting to the service component | 
**kafka_authentication_method** | Option<[**crate::models::EnumKafkaAuthMethod**](enum-kafka-auth-method.md)> |  | [optional]
**path** | Option<**String**> | Path component of the service URL (useful only if service component is HTTP or HTTPS endpoint) | [optional]
**port** | **i64** | Port number for connecting to the service component | 
**route** | **String** | Network access route | 
**ssl** | Option<**bool**> | Whether the endpoint is encrypted or accepts plaintext.                                            By default endpoints are always encrypted and                                            this property is only included for service components that may disable encryption. | [optional]
**usage** | **String** | DNS usage name | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


