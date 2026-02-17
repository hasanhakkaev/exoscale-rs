# DbaasServiceKafka

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**authentication_methods** | Option<[**models::DbaasServiceKafkaAuthenticationMethods**](DbaasServiceKafkaAuthenticationMethods.md)> |  | [optional]
**node_count** | Option<**u64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**models::DbaasServiceKafkaConnectionInfo**](DbaasServiceKafkaConnectionInfo.md)> |  | [optional]
**node_cpu_count** | Option<**u64**> | Number of CPUs for each node | [optional]
**kafka_rest_enabled** | Option<**bool**> | Whether Kafka REST is enabled | [optional]
**prometheus_uri** | [**models::DbaasServiceMysqlPrometheusUri**](DbaasServiceMysqlPrometheusUri.md) |  | 
**integrations** | Option<[**Vec<models::DbaasIntegration>**](DbaasIntegration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**node_states** | Option<[**Vec<models::DbaasNodeState>**](DbaasNodeState.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**kafka_connect_enabled** | Option<**bool**> | Whether Kafka Connect is enabled | [optional]
**r#type** | **String** |  | 
**state** | Option<[**models::EnumServiceState**](EnumServiceState.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**schema_registry_settings** | Option<[**models::JsonSchemaSchemaRegistry**](JsonSchemaSchemaRegistry.md)> |  | [optional]
**backups** | Option<[**Vec<models::DbaasServiceBackup>**](DbaasServiceBackup.md)> | List of backups for the service | [optional]
**kafka_rest_settings** | Option<[**models::JsonSchemaKafkaRest**](JsonSchemaKafkaRest.md)> |  | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<models::DbaasServiceNotification>**](DbaasServiceNotification.md)> | Service notifications | [optional]
**kafka_connect_settings** | Option<[**models::JsonSchemaKafkaConnect**](JsonSchemaKafkaConnect.md)> |  | [optional]
**components** | Option<[**Vec<models::DbaasServiceKafkaComponentsInner>**](DbaasServiceKafkaComponentsInner.md)> | Service component information objects | [optional]
**maintenance** | Option<[**models::DbaasServiceMaintenance**](DbaasServiceMaintenance.md)> |  | [optional]
**kafka_settings** | Option<[**models::JsonSchemaKafka**](JsonSchemaKafka.md)> |  | [optional]
**disk_size** | Option<**u64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**u64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<**serde_json::Value**> | service_uri parameterized into key-value pairs | [optional]
**schema_registry_enabled** | Option<**bool**> | Whether Schema-Registry is enabled | [optional]
**version** | Option<**String**> | Kafka version | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**users** | Option<[**Vec<models::DbaasServiceKafkaUsersInner>**](DbaasServiceKafkaUsersInner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


