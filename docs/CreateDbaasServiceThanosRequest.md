# CreateDbaasServiceThanosRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](UpdateDbaasServiceMysqlRequestMaintenance.md)> |  | [optional]
**plan** | **String** | Subscription plan | 
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**ip_filter** | Option<**Vec<String>**> | Allowed CIDR address blocks for incoming connections | [optional]
**thanos_settings** | Option<[**models::JsonSchemaThanos**](JsonSchemaThanos.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


