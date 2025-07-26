# DbaasServiceThanos

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**u64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**models::DbaasServiceThanosConnectionInfo**](dbaas_service_thanos_connection_info.md)> |  | [optional]
**node_cpu_count** | Option<**u64**> | Number of CPUs for each node | [optional]
**prometheus_uri** | [**models::DbaasServiceMysqlPrometheusUri**](dbaas_service_mysql_prometheus_uri.md) |  | 
**integrations** | Option<[**Vec<models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**node_states** | Option<[**Vec<models::DbaasNodeState>**](dbaas-node-state.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**r#type** | **String** |  | 
**state** | Option<[**models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**backups** | Option<[**Vec<models::DbaasServiceBackup>**](dbaas-service-backup.md)> | List of backups for the service | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**components** | Option<[**Vec<models::DbaasServiceThanosComponentsInner>**](dbaas_service_thanos_components_inner.md)> | Service component information objects | [optional]
**maintenance** | Option<[**models::DbaasServiceMaintenance**](dbaas-service-maintenance.md)> |  | [optional]
**disk_size** | Option<**u64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**u64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<[**serde_json::Value**](.md)> | service_uri parameterized into key-value pairs | [optional]
**thanos_settings** | Option<[**models::JsonSchemaThanos**](json-schema-thanos.md)> |  | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


