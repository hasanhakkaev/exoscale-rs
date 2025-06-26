# DbaasServiceValkeyComponentsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**component** | **String** | Service component name | 
**host** | **String** | DNS name for connecting to the service component | 
**port** | **u64** | Port number for connecting to the service component | 
**route** | [**models::EnumComponentRoute**](enum-component-route.md) |  | 
**ssl** | Option<**bool**> | Whether the endpoint is encrypted or accepts plaintext.              By default endpoints are always encrypted and              this property is only included for service components that may disable encryption. | [optional]
**usage** | [**models::EnumComponentUsage**](enum-component-usage.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


