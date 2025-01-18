# DbaasServiceCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**i64**> | Number of service nodes in the active plan | [optional]
**node_cpu_count** | Option<**i64**> | Number of CPUs for each node | [optional]
**integrations** | Option<[**Vec<models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**name** | **String** |  | 
**r#type** | **String** |  | 
**state** | Option<[**models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**disk_size** | Option<**i64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**i64**> | TODO UNIT of memory for each node | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


