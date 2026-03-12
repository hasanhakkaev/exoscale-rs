# DbaasServiceValkey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**u64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**models::DbaasServiceValkeyConnectionInfo**](DbaasServiceValkeyConnectionInfo.md)> |  | [optional]
**node_cpu_count** | Option<**u64**> | Number of CPUs for each node | [optional]
**prometheus_uri** | [**models::DbaasServiceMysqlPrometheusUri**](DbaasServiceMysqlPrometheusUri.md) |  | 
**integrations** | Option<[**Vec<models::DbaasIntegration>**](DbaasIntegration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**node_states** | Option<[**Vec<models::DbaasNodeState>**](DbaasNodeState.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**r#type** | **String** |  | 
**state** | Option<[**models::EnumServiceState**](EnumServiceState.md)> |  | [optional]
**valkey_settings** | Option<[**models::JsonSchemaValkey**](JsonSchemaValkey.md)> |  | [optional]
**backups** | Option<[**Vec<models::DbaasServiceBackup>**](DbaasServiceBackup.md)> | List of backups for the service | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<models::DbaasServiceNotification>**](DbaasServiceNotification.md)> | Service notifications | [optional]
**components** | Option<[**Vec<models::DbaasServiceThanosComponentsInner>**](DbaasServiceThanosComponentsInner.md)> | Service component information objects | [optional]
**maintenance** | Option<[**models::DbaasServiceMaintenance**](DbaasServiceMaintenance.md)> |  | [optional]
**disk_size** | Option<**u64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**u64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<**serde_json::Value**> | service_uri parameterized into key-value pairs | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**users** | Option<[**Vec<models::DbaasServiceValkeyUsersInner>**](DbaasServiceValkeyUsersInner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


