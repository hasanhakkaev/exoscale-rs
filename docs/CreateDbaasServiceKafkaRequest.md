# CreateDbaasServiceKafkaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_methods** | Option<[**models::UpdateDbaasServiceKafkaRequestAuthenticationMethods**](update_dbaas_service_kafka_request_authentication_methods.md)> |  | [optional]
**kafka_rest_enabled** | Option<**bool**> | Enable Kafka-REST service | [optional]
**kafka_connect_enabled** | Option<**bool**> | Allow clients to connect to kafka_connect from the public internet for service nodes that are in a project VPC or another type of private network | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**schema_registry_settings** | Option<[**models::JsonSchemaSchemaRegistry**](json-schema-schema-registry.md)> |  | [optional]
**kafka_rest_settings** | Option<[**models::JsonSchemaKafkaRest**](json-schema-kafka-rest.md)> |  | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**kafka_connect_settings** | Option<[**models::JsonSchemaKafkaConnect**](json-schema-kafka-connect.md)> |  | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**kafka_settings** | Option<[**models::JsonSchemaKafka**](json-schema-kafka.md)> |  | [optional]
**schema_registry_enabled** | Option<**bool**> | Enable Schema-Registry service | [optional]
**version** | Option<**String**> | Kafka major version | [optional]
**plan** | **String** | Subscription plan | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


