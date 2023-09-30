# DbaasServiceIntegration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**integration_status** | Option<[**serde_json::Value**](.md)> | Integration status | [optional]
**description** | **String** | Description of the integration | 
**source_service_type** | **String** |  | 
**source_endpoint** | Option<**String**> | Source endpoint name | [optional]
**dest_service_type** | **String** |  | 
**integration_type** | **String** | Type of the integration | 
**dest_endpoint** | Option<**String**> | Destination endpoint name | [optional]
**user_config** | Option<[**serde_json::Value**](.md)> | Service integration settings | [optional]
**dest_endpoint_id** | Option<**String**> | Destination endpoint id | [optional]
**service_integration_id** | **String** | Integration ID | 
**dest_service** | **String** | Destination service name | 
**active** | **bool** | True when integration is active | 
**source_endpoint_id** | Option<**String**> | Source endpoint ID | [optional]
**source_service** | **String** | Source service name | 
**enabled** | **bool** | True when integration is enabled | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


