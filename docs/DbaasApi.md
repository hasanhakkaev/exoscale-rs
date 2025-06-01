# \DbaasApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_dbaas_service_to_endpoint**](DbaasApi.md#attach_dbaas_service_to_endpoint) | **PUT** /dbaas-external-endpoint/{source_service_name}/attach | 
[**create_dbaas_external_endpoint_datadog**](DbaasApi.md#create_dbaas_external_endpoint_datadog) | **POST** /dbaas-external-endpoint-datadog/{name} | 
[**create_dbaas_external_endpoint_elasticsearch**](DbaasApi.md#create_dbaas_external_endpoint_elasticsearch) | **POST** /dbaas-external-endpoint-elasticsearch/{name} | 
[**create_dbaas_external_endpoint_opensearch**](DbaasApi.md#create_dbaas_external_endpoint_opensearch) | **POST** /dbaas-external-endpoint-opensearch/{name} | 
[**create_dbaas_external_endpoint_prometheus**](DbaasApi.md#create_dbaas_external_endpoint_prometheus) | **POST** /dbaas-external-endpoint-prometheus/{name} | 
[**create_dbaas_external_endpoint_rsyslog**](DbaasApi.md#create_dbaas_external_endpoint_rsyslog) | **POST** /dbaas-external-endpoint-rsyslog/{name} | 
[**create_dbaas_integration**](DbaasApi.md#create_dbaas_integration) | **POST** /dbaas-integration | 
[**create_dbaas_kafka_schema_registry_acl_config**](DbaasApi.md#create_dbaas_kafka_schema_registry_acl_config) | **POST** /dbaas-kafka/{name}/schema-registry/acl-config | Add a Kafka Schema Registry ACL entry
[**create_dbaas_kafka_topic_acl_config**](DbaasApi.md#create_dbaas_kafka_topic_acl_config) | **POST** /dbaas-kafka/{name}/topic/acl-config | Add a Kafka topic ACL entry
[**create_dbaas_kafka_user**](DbaasApi.md#create_dbaas_kafka_user) | **POST** /dbaas-kafka/{service_name}/user | Create a DBaaS Kafka user
[**create_dbaas_mysql_database**](DbaasApi.md#create_dbaas_mysql_database) | **POST** /dbaas-mysql/{service_name}/database | Create a DBaaS MySQL database
[**create_dbaas_mysql_user**](DbaasApi.md#create_dbaas_mysql_user) | **POST** /dbaas-mysql/{service_name}/user | Create a DBaaS MySQL user
[**create_dbaas_opensearch_user**](DbaasApi.md#create_dbaas_opensearch_user) | **POST** /dbaas-opensearch/{service_name}/user | Create a DBaaS OpenSearch user
[**create_dbaas_pg_connection_pool**](DbaasApi.md#create_dbaas_pg_connection_pool) | **POST** /dbaas-postgres/{service_name}/connection-pool | Create a DBaaS PostgreSQL connection pool
[**create_dbaas_pg_database**](DbaasApi.md#create_dbaas_pg_database) | **POST** /dbaas-postgres/{service_name}/database | Create a DBaaS Postgres database
[**create_dbaas_pg_upgrade_check**](DbaasApi.md#create_dbaas_pg_upgrade_check) | **POST** /dbaas-postgres/{service}/upgrade-check | 
[**create_dbaas_postgres_user**](DbaasApi.md#create_dbaas_postgres_user) | **POST** /dbaas-postgres/{service_name}/user | Create a DBaaS Postgres user
[**create_dbaas_redis_user**](DbaasApi.md#create_dbaas_redis_user) | **POST** /dbaas-redis/{service_name}/user | Create a DBaaS Redis user
[**create_dbaas_service_grafana**](DbaasApi.md#create_dbaas_service_grafana) | **POST** /dbaas-grafana/{name} | 
[**create_dbaas_service_kafka**](DbaasApi.md#create_dbaas_service_kafka) | **POST** /dbaas-kafka/{name} | Create a DBaaS Kafka service
[**create_dbaas_service_mysql**](DbaasApi.md#create_dbaas_service_mysql) | **POST** /dbaas-mysql/{name} | Create a DBaaS MySQL service
[**create_dbaas_service_opensearch**](DbaasApi.md#create_dbaas_service_opensearch) | **POST** /dbaas-opensearch/{name} | Create a DBaaS OpenSearch service
[**create_dbaas_service_pg**](DbaasApi.md#create_dbaas_service_pg) | **POST** /dbaas-postgres/{name} | Create a DBaaS PostgreSQL service
[**create_dbaas_service_redis**](DbaasApi.md#create_dbaas_service_redis) | **POST** /dbaas-redis/{name} | Create a DBaaS Redis service
[**create_dbaas_service_valkey**](DbaasApi.md#create_dbaas_service_valkey) | **POST** /dbaas-valkey/{name} | Create a DBaaS Valkey service
[**create_dbaas_task_migration_check**](DbaasApi.md#create_dbaas_task_migration_check) | **POST** /dbaas-task-migration-check/{service} | 
[**create_dbaas_valkey_user**](DbaasApi.md#create_dbaas_valkey_user) | **POST** /dbaas-valkey/{service_name}/user | Create a DBaaS Valkey user
[**delete_dbaas_external_endpoint_datadog**](DbaasApi.md#delete_dbaas_external_endpoint_datadog) | **DELETE** /dbaas-external-endpoint-datadog/{endpoint_id} | 
[**delete_dbaas_external_endpoint_elasticsearch**](DbaasApi.md#delete_dbaas_external_endpoint_elasticsearch) | **DELETE** /dbaas-external-endpoint-elasticsearch/{endpoint_id} | 
[**delete_dbaas_external_endpoint_opensearch**](DbaasApi.md#delete_dbaas_external_endpoint_opensearch) | **DELETE** /dbaas-external-endpoint-opensearch/{endpoint_id} | 
[**delete_dbaas_external_endpoint_prometheus**](DbaasApi.md#delete_dbaas_external_endpoint_prometheus) | **DELETE** /dbaas-external-endpoint-prometheus/{endpoint_id} | 
[**delete_dbaas_external_endpoint_rsyslog**](DbaasApi.md#delete_dbaas_external_endpoint_rsyslog) | **DELETE** /dbaas-external-endpoint-rsyslog/{endpoint_id} | 
[**delete_dbaas_integration**](DbaasApi.md#delete_dbaas_integration) | **DELETE** /dbaas-integration/{id} | 
[**delete_dbaas_kafka_schema_registry_acl_config**](DbaasApi.md#delete_dbaas_kafka_schema_registry_acl_config) | **DELETE** /dbaas-kafka/{name}/schema-registry/acl-config/{acl_id} | Delete a Kafka ACL entry
[**delete_dbaas_kafka_topic_acl_config**](DbaasApi.md#delete_dbaas_kafka_topic_acl_config) | **DELETE** /dbaas-kafka/{name}/topic/acl-config/{acl_id} | Delete a Kafka ACL entry
[**delete_dbaas_kafka_user**](DbaasApi.md#delete_dbaas_kafka_user) | **DELETE** /dbaas-kafka/{service_name}/user/{username} | Delete a DBaaS kafka user
[**delete_dbaas_mysql_database**](DbaasApi.md#delete_dbaas_mysql_database) | **DELETE** /dbaas-mysql/{service_name}/database/{database_name} | Delete a DBaaS MySQL database
[**delete_dbaas_mysql_user**](DbaasApi.md#delete_dbaas_mysql_user) | **DELETE** /dbaas-mysql/{service_name}/user/{username} | Delete a DBaaS MySQL user
[**delete_dbaas_opensearch_user**](DbaasApi.md#delete_dbaas_opensearch_user) | **DELETE** /dbaas-opensearch/{service_name}/user/{username} | Delete a DBaaS OpenSearch user
[**delete_dbaas_pg_connection_pool**](DbaasApi.md#delete_dbaas_pg_connection_pool) | **DELETE** /dbaas-postgres/{service_name}/connection-pool/{connection_pool_name} | Delete a DBaaS PostgreSQL connection pool
[**delete_dbaas_pg_database**](DbaasApi.md#delete_dbaas_pg_database) | **DELETE** /dbaas-postgres/{service_name}/database/{database_name} | Delete a DBaaS Postgres database
[**delete_dbaas_postgres_user**](DbaasApi.md#delete_dbaas_postgres_user) | **DELETE** /dbaas-postgres/{service_name}/user/{username} | Delete a DBaaS Postgres user
[**delete_dbaas_redis_user**](DbaasApi.md#delete_dbaas_redis_user) | **DELETE** /dbaas-redis/{service_name}/user/{username} | Delete a DBaaS Redis user
[**delete_dbaas_service**](DbaasApi.md#delete_dbaas_service) | **DELETE** /dbaas-service/{name} | Delete a DBaaS service
[**delete_dbaas_service_grafana**](DbaasApi.md#delete_dbaas_service_grafana) | **DELETE** /dbaas-grafana/{name} | Delete a Grafana service
[**delete_dbaas_service_kafka**](DbaasApi.md#delete_dbaas_service_kafka) | **DELETE** /dbaas-kafka/{name} | Delete a Kafka service
[**delete_dbaas_service_mysql**](DbaasApi.md#delete_dbaas_service_mysql) | **DELETE** /dbaas-mysql/{name} | Delete a MySQL service
[**delete_dbaas_service_opensearch**](DbaasApi.md#delete_dbaas_service_opensearch) | **DELETE** /dbaas-opensearch/{name} | Delete a OpenSearch service
[**delete_dbaas_service_pg**](DbaasApi.md#delete_dbaas_service_pg) | **DELETE** /dbaas-postgres/{name} | Delete a Postgres service
[**delete_dbaas_service_redis**](DbaasApi.md#delete_dbaas_service_redis) | **DELETE** /dbaas-redis/{name} | Delete a Redis service
[**delete_dbaas_service_valkey**](DbaasApi.md#delete_dbaas_service_valkey) | **DELETE** /dbaas-valkey/{name} | Delete a Valkey service
[**delete_dbaas_valkey_user**](DbaasApi.md#delete_dbaas_valkey_user) | **DELETE** /dbaas-valkey/{service_name}/user/{username} | Delete a DBaaS Valkey user
[**detach_dbaas_service_from_endpoint**](DbaasApi.md#detach_dbaas_service_from_endpoint) | **PUT** /dbaas-external-endpoint/{source_service_name}/detach | 
[**enable_dbaas_mysql_writes**](DbaasApi.md#enable_dbaas_mysql_writes) | **PUT** /dbaas-mysql/{name}/enable/writes | Temporarily enable writes for MySQL services in read-only mode due to filled up storage
[**get_dbaas_ca_certificate**](DbaasApi.md#get_dbaas_ca_certificate) | **GET** /dbaas-ca-certificate | Get DBaaS CA Certificate
[**get_dbaas_external_endpoint_datadog**](DbaasApi.md#get_dbaas_external_endpoint_datadog) | **GET** /dbaas-external-endpoint-datadog/{endpoint_id} | 
[**get_dbaas_external_endpoint_elasticsearch**](DbaasApi.md#get_dbaas_external_endpoint_elasticsearch) | **GET** /dbaas-external-endpoint-elasticsearch/{endpoint_id} | 
[**get_dbaas_external_endpoint_opensearch**](DbaasApi.md#get_dbaas_external_endpoint_opensearch) | **GET** /dbaas-external-endpoint-opensearch/{endpoint_id} | 
[**get_dbaas_external_endpoint_prometheus**](DbaasApi.md#get_dbaas_external_endpoint_prometheus) | **GET** /dbaas-external-endpoint-prometheus/{endpoint_id} | 
[**get_dbaas_external_endpoint_rsyslog**](DbaasApi.md#get_dbaas_external_endpoint_rsyslog) | **GET** /dbaas-external-endpoint-rsyslog/{endpoint_id} | 
[**get_dbaas_external_integration**](DbaasApi.md#get_dbaas_external_integration) | **GET** /dbaas-external-integration/{integration_id} | 
[**get_dbaas_external_integration_settings_datadog**](DbaasApi.md#get_dbaas_external_integration_settings_datadog) | **GET** /dbaas-external-integration-settings-datadog/{integration_id} | 
[**get_dbaas_integration**](DbaasApi.md#get_dbaas_integration) | **GET** /dbaas-integration/{id} | 
[**get_dbaas_kafka_acl_config**](DbaasApi.md#get_dbaas_kafka_acl_config) | **GET** /dbaas-kafka/{name}/acl-config | Get DBaaS kafka ACL configuration
[**get_dbaas_migration_status**](DbaasApi.md#get_dbaas_migration_status) | **GET** /dbaas-migration-status/{name} | Get a DBaaS migration status
[**get_dbaas_opensearch_acl_config**](DbaasApi.md#get_dbaas_opensearch_acl_config) | **GET** /dbaas-opensearch/{name}/acl-config | Get DBaaS OpenSearch ACL configuration
[**get_dbaas_service_grafana**](DbaasApi.md#get_dbaas_service_grafana) | **GET** /dbaas-grafana/{name} | Get a DBaaS Grafana service
[**get_dbaas_service_kafka**](DbaasApi.md#get_dbaas_service_kafka) | **GET** /dbaas-kafka/{name} | Get a DBaaS Kafka service
[**get_dbaas_service_logs**](DbaasApi.md#get_dbaas_service_logs) | **POST** /dbaas-service-logs/{service_name} | Get logs of DBaaS service
[**get_dbaas_service_metrics**](DbaasApi.md#get_dbaas_service_metrics) | **POST** /dbaas-service-metrics/{service_name} | Get metrics of DBaaS service
[**get_dbaas_service_mysql**](DbaasApi.md#get_dbaas_service_mysql) | **GET** /dbaas-mysql/{name} | Get a DBaaS MySQL service
[**get_dbaas_service_opensearch**](DbaasApi.md#get_dbaas_service_opensearch) | **GET** /dbaas-opensearch/{name} | Get a DBaaS OpenSearch service
[**get_dbaas_service_pg**](DbaasApi.md#get_dbaas_service_pg) | **GET** /dbaas-postgres/{name} | Get a DBaaS PostgreSQL service
[**get_dbaas_service_redis**](DbaasApi.md#get_dbaas_service_redis) | **GET** /dbaas-redis/{name} | Get a DBaaS Redis service
[**get_dbaas_service_type**](DbaasApi.md#get_dbaas_service_type) | **GET** /dbaas-service-type/{service_type_name} | Get a DBaaS service type
[**get_dbaas_service_valkey**](DbaasApi.md#get_dbaas_service_valkey) | **GET** /dbaas-valkey/{name} | 
[**get_dbaas_settings_grafana**](DbaasApi.md#get_dbaas_settings_grafana) | **GET** /dbaas-settings-grafana | Get DBaaS Grafana settings
[**get_dbaas_settings_kafka**](DbaasApi.md#get_dbaas_settings_kafka) | **GET** /dbaas-settings-kafka | Get DBaaS Kafka settings
[**get_dbaas_settings_mysql**](DbaasApi.md#get_dbaas_settings_mysql) | **GET** /dbaas-settings-mysql | Get DBaaS MySQL settings
[**get_dbaas_settings_opensearch**](DbaasApi.md#get_dbaas_settings_opensearch) | **GET** /dbaas-settings-opensearch | Get DBaaS OpenSearch settings
[**get_dbaas_settings_pg**](DbaasApi.md#get_dbaas_settings_pg) | **GET** /dbaas-settings-pg | Get DBaaS PostgreSQL settings
[**get_dbaas_settings_redis**](DbaasApi.md#get_dbaas_settings_redis) | **GET** /dbaas-settings-redis | Get DBaaS Redis settings
[**get_dbaas_settings_valkey**](DbaasApi.md#get_dbaas_settings_valkey) | **GET** /dbaas-settings-valkey | Get DBaaS Valkey settings
[**get_dbaas_task**](DbaasApi.md#get_dbaas_task) | **GET** /dbaas-task/{service}/{id} | Get a DBaaS task
[**list_dbaas_external_endpoint_types**](DbaasApi.md#list_dbaas_external_endpoint_types) | **GET** /dbaas-external-endpoint-types | 
[**list_dbaas_external_endpoints**](DbaasApi.md#list_dbaas_external_endpoints) | **GET** /dbaas-external-endpoints | 
[**list_dbaas_external_integrations**](DbaasApi.md#list_dbaas_external_integrations) | **GET** /dbaas-external-integrations/{service_name} | 
[**list_dbaas_integration_settings**](DbaasApi.md#list_dbaas_integration_settings) | **GET** /dbaas-integration-settings/{integration_type}/{source_type}/{dest_type} | 
[**list_dbaas_integration_types**](DbaasApi.md#list_dbaas_integration_types) | **GET** /dbaas-integration-types | 
[**list_dbaas_service_types**](DbaasApi.md#list_dbaas_service_types) | **GET** /dbaas-service-type | DBaaS Service Types
[**list_dbaas_services**](DbaasApi.md#list_dbaas_services) | **GET** /dbaas-service | List DBaaS services
[**reset_dbaas_grafana_user_password**](DbaasApi.md#reset_dbaas_grafana_user_password) | **PUT** /dbaas-grafana/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Grafana user
[**reset_dbaas_kafka_user_password**](DbaasApi.md#reset_dbaas_kafka_user_password) | **PUT** /dbaas-kafka/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Kafka user
[**reset_dbaas_mysql_user_password**](DbaasApi.md#reset_dbaas_mysql_user_password) | **PUT** /dbaas-mysql/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS mysql user
[**reset_dbaas_opensearch_user_password**](DbaasApi.md#reset_dbaas_opensearch_user_password) | **PUT** /dbaas-opensearch/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS OpenSearch user
[**reset_dbaas_postgres_user_password**](DbaasApi.md#reset_dbaas_postgres_user_password) | **PUT** /dbaas-postgres/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Postgres user
[**reset_dbaas_redis_user_password**](DbaasApi.md#reset_dbaas_redis_user_password) | **PUT** /dbaas-redis/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Redis user
[**reset_dbaas_valkey_user_password**](DbaasApi.md#reset_dbaas_valkey_user_password) | **PUT** /dbaas-valkey/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Valkey user
[**reveal_dbaas_grafana_user_password**](DbaasApi.md#reveal_dbaas_grafana_user_password) | **GET** /dbaas-grafana/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS Grafana user
[**reveal_dbaas_kafka_connect_password**](DbaasApi.md#reveal_dbaas_kafka_connect_password) | **GET** /dbaas-kafka/{service_name}/connect/password/reveal | Reveal the secrets for DBaaS Kafka Connect
[**reveal_dbaas_kafka_user_password**](DbaasApi.md#reveal_dbaas_kafka_user_password) | **GET** /dbaas-kafka/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS Kafka user
[**reveal_dbaas_mysql_user_password**](DbaasApi.md#reveal_dbaas_mysql_user_password) | **GET** /dbaas-mysql/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS MySQL user
[**reveal_dbaas_opensearch_user_password**](DbaasApi.md#reveal_dbaas_opensearch_user_password) | **GET** /dbaas-opensearch/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS OpenSearch user
[**reveal_dbaas_postgres_user_password**](DbaasApi.md#reveal_dbaas_postgres_user_password) | **GET** /dbaas-postgres/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS Postgres user
[**reveal_dbaas_redis_user_password**](DbaasApi.md#reveal_dbaas_redis_user_password) | **GET** /dbaas-redis/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS Redis user
[**reveal_dbaas_valkey_user_password**](DbaasApi.md#reveal_dbaas_valkey_user_password) | **GET** /dbaas-valkey/{service_name}/user/{username}/password/reveal | Reveal the secrets of a DBaaS Valkey user
[**start_dbaas_grafana_maintenance**](DbaasApi.md#start_dbaas_grafana_maintenance) | **PUT** /dbaas-grafana/{name}/maintenance/start | Initiate Grafana maintenance update
[**start_dbaas_kafka_maintenance**](DbaasApi.md#start_dbaas_kafka_maintenance) | **PUT** /dbaas-kafka/{name}/maintenance/start | Initiate Kafka maintenance update
[**start_dbaas_mysql_maintenance**](DbaasApi.md#start_dbaas_mysql_maintenance) | **PUT** /dbaas-mysql/{name}/maintenance/start | Initiate MySQL maintenance update
[**start_dbaas_opensearch_maintenance**](DbaasApi.md#start_dbaas_opensearch_maintenance) | **PUT** /dbaas-opensearch/{name}/maintenance/start | Initiate OpenSearch maintenance update
[**start_dbaas_pg_maintenance**](DbaasApi.md#start_dbaas_pg_maintenance) | **PUT** /dbaas-postgres/{name}/maintenance/start | Initiate PostgreSQL maintenance update
[**start_dbaas_redis_maintenance**](DbaasApi.md#start_dbaas_redis_maintenance) | **PUT** /dbaas-redis/{name}/maintenance/start | Initiate Redis maintenance update
[**start_dbaas_redis_to_valkey_upgrade**](DbaasApi.md#start_dbaas_redis_to_valkey_upgrade) | **PUT** /dbaas-redis/{name}/upgrade-type | Initiate Redis upgrade to Valkey
[**start_dbaas_valkey_maintenance**](DbaasApi.md#start_dbaas_valkey_maintenance) | **PUT** /dbaas-valkey/{name}/maintenance/start | Initiate Valkey maintenance update
[**stop_dbaas_mysql_migration**](DbaasApi.md#stop_dbaas_mysql_migration) | **POST** /dbaas-mysql/{name}/migration/stop | Stop a DBaaS MySQL migration
[**stop_dbaas_pg_migration**](DbaasApi.md#stop_dbaas_pg_migration) | **POST** /dbaas-postgres/{name}/migration/stop | Stop a DBaaS PostgreSQL migration
[**stop_dbaas_redis_migration**](DbaasApi.md#stop_dbaas_redis_migration) | **POST** /dbaas-redis/{name}/migration/stop | Stop a DBaaS Redis migration
[**stop_dbaas_valkey_migration**](DbaasApi.md#stop_dbaas_valkey_migration) | **POST** /dbaas-valkey/{name}/migration/stop | Stop a DBaaS Valkey migration
[**update_dbaas_external_endpoint_datadog**](DbaasApi.md#update_dbaas_external_endpoint_datadog) | **PUT** /dbaas-external-endpoint-datadog/{endpoint_id} | 
[**update_dbaas_external_endpoint_elasticsearch**](DbaasApi.md#update_dbaas_external_endpoint_elasticsearch) | **PUT** /dbaas-external-endpoint-elasticsearch/{endpoint_id} | 
[**update_dbaas_external_endpoint_opensearch**](DbaasApi.md#update_dbaas_external_endpoint_opensearch) | **PUT** /dbaas-external-endpoint-opensearch/{endpoint_id} | 
[**update_dbaas_external_endpoint_prometheus**](DbaasApi.md#update_dbaas_external_endpoint_prometheus) | **PUT** /dbaas-external-endpoint-prometheus/{endpoint_id} | 
[**update_dbaas_external_endpoint_rsyslog**](DbaasApi.md#update_dbaas_external_endpoint_rsyslog) | **PUT** /dbaas-external-endpoint-rsyslog/{endpoint_id} | 
[**update_dbaas_external_integration_settings_datadog**](DbaasApi.md#update_dbaas_external_integration_settings_datadog) | **POST** /dbaas-external-integration-settings-datadog/{integration_id} | 
[**update_dbaas_integration**](DbaasApi.md#update_dbaas_integration) | **PUT** /dbaas-integration/{id} | 
[**update_dbaas_opensearch_acl_config**](DbaasApi.md#update_dbaas_opensearch_acl_config) | **PUT** /dbaas-opensearch/{name}/acl-config | Create a DBaaS OpenSearch ACL configuration
[**update_dbaas_pg_connection_pool**](DbaasApi.md#update_dbaas_pg_connection_pool) | **PUT** /dbaas-postgres/{service_name}/connection-pool/{connection_pool_name} | Update a DBaaS PostgreSQL connection pool
[**update_dbaas_postgres_allow_replication**](DbaasApi.md#update_dbaas_postgres_allow_replication) | **PUT** /dbaas-postgres/{service_name}/user/{username}/allow-replication | Update access control for one service user
[**update_dbaas_service_grafana**](DbaasApi.md#update_dbaas_service_grafana) | **PUT** /dbaas-grafana/{name} | Update a DBaaS Grafana service
[**update_dbaas_service_kafka**](DbaasApi.md#update_dbaas_service_kafka) | **PUT** /dbaas-kafka/{name} | Update a DBaaS Kafka service
[**update_dbaas_service_mysql**](DbaasApi.md#update_dbaas_service_mysql) | **PUT** /dbaas-mysql/{name} | Update a DBaaS MySQL service
[**update_dbaas_service_opensearch**](DbaasApi.md#update_dbaas_service_opensearch) | **PUT** /dbaas-opensearch/{name} | Update a DBaaS OpenSearch service
[**update_dbaas_service_pg**](DbaasApi.md#update_dbaas_service_pg) | **PUT** /dbaas-postgres/{name} | Update a DBaaS PostgreSQL service
[**update_dbaas_service_redis**](DbaasApi.md#update_dbaas_service_redis) | **PUT** /dbaas-redis/{name} | Update a DBaaS Redis service
[**update_dbaas_service_valkey**](DbaasApi.md#update_dbaas_service_valkey) | **PUT** /dbaas-valkey/{name} | 



## attach_dbaas_service_to_endpoint

> models::Operation attach_dbaas_service_to_endpoint(source_service_name, attach_dbaas_service_to_endpoint_request)


[BETA] Create a new DBaaS connection between a DBaaS service and an external service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_service_name** | **String** |  | [required] |
**attach_dbaas_service_to_endpoint_request** | [**AttachDbaasServiceToEndpointRequest**](AttachDbaasServiceToEndpointRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_external_endpoint_datadog

> models::Operation create_dbaas_external_endpoint_datadog(name, dbaas_endpoint_datadog_input_create)


[BETA] Create DataDog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_endpoint_datadog_input_create** | [**DbaasEndpointDatadogInputCreate**](DbaasEndpointDatadogInputCreate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_external_endpoint_elasticsearch

> models::Operation create_dbaas_external_endpoint_elasticsearch(name, dbaas_endpoint_elasticsearch_input_create)


[BETA] Create ElasticSearch Logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_endpoint_elasticsearch_input_create** | [**DbaasEndpointElasticsearchInputCreate**](DbaasEndpointElasticsearchInputCreate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_external_endpoint_opensearch

> models::Operation create_dbaas_external_endpoint_opensearch(name, dbaas_endpoint_opensearch_input_create)


[BETA] Create OpenSearch Logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_endpoint_opensearch_input_create** | [**DbaasEndpointOpensearchInputCreate**](DbaasEndpointOpensearchInputCreate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_external_endpoint_prometheus

> models::Operation create_dbaas_external_endpoint_prometheus(name, dbaas_endpoint_prometheus_payload)


[BETA] Create Prometheus external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_endpoint_prometheus_payload** | [**DbaasEndpointPrometheusPayload**](DbaasEndpointPrometheusPayload.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_external_endpoint_rsyslog

> models::Operation create_dbaas_external_endpoint_rsyslog(name, dbaas_endpoint_rsyslog_input_create)


[BETA] Create RSyslog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_endpoint_rsyslog_input_create** | [**DbaasEndpointRsyslogInputCreate**](DbaasEndpointRsyslogInputCreate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_integration

> models::Operation create_dbaas_integration(create_dbaas_integration_request)


[BETA] Create a new DBaaS integration between two services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dbaas_integration_request** | [**CreateDbaasIntegrationRequest**](CreateDbaasIntegrationRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_schema_registry_acl_config

> models::Operation create_dbaas_kafka_schema_registry_acl_config(name, dbaas_kafka_schema_registry_acl_entry)
Add a Kafka Schema Registry ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_kafka_schema_registry_acl_entry** | [**DbaasKafkaSchemaRegistryAclEntry**](DbaasKafkaSchemaRegistryAclEntry.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_topic_acl_config

> models::Operation create_dbaas_kafka_topic_acl_config(name, dbaas_kafka_topic_acl_entry)
Add a Kafka topic ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_kafka_topic_acl_entry** | [**DbaasKafkaTopicAclEntry**](DbaasKafkaTopicAclEntry.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_user

> models::Operation create_dbaas_kafka_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS Kafka user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_mysql_database

> models::Operation create_dbaas_mysql_database(service_name, create_dbaas_mysql_database_request)
Create a DBaaS MySQL database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_mysql_database_request** | [**CreateDbaasMysqlDatabaseRequest**](CreateDbaasMysqlDatabaseRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_mysql_user

> models::Operation create_dbaas_mysql_user(service_name, create_dbaas_mysql_user_request)
Create a DBaaS MySQL user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_mysql_user_request** | [**CreateDbaasMysqlUserRequest**](CreateDbaasMysqlUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_opensearch_user

> models::Operation create_dbaas_opensearch_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS OpenSearch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_connection_pool

> models::Operation create_dbaas_pg_connection_pool(service_name, create_dbaas_pg_connection_pool_request)
Create a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_pg_connection_pool_request** | [**CreateDbaasPgConnectionPoolRequest**](CreateDbaasPgConnectionPoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_database

> models::Operation create_dbaas_pg_database(service_name, create_dbaas_pg_database_request)
Create a DBaaS Postgres database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_pg_database_request** | [**CreateDbaasPgDatabaseRequest**](CreateDbaasPgDatabaseRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_upgrade_check

> models::DbaasTask create_dbaas_pg_upgrade_check(service, create_dbaas_pg_upgrade_check_request)


Check whether you can upgrade Postgres service to a newer version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**create_dbaas_pg_upgrade_check_request** | [**CreateDbaasPgUpgradeCheckRequest**](CreateDbaasPgUpgradeCheckRequest.md) |  | [required] |

### Return type

[**models::DbaasTask**](dbaas-task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_postgres_user

> models::Operation create_dbaas_postgres_user(service_name, create_dbaas_postgres_user_request)
Create a DBaaS Postgres user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_postgres_user_request** | [**CreateDbaasPostgresUserRequest**](CreateDbaasPostgresUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_redis_user

> models::Operation create_dbaas_redis_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS Redis user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_grafana

> models::Operation create_dbaas_service_grafana(name, create_dbaas_service_grafana_request)


Create a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_grafana_request** | [**CreateDbaasServiceGrafanaRequest**](CreateDbaasServiceGrafanaRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_kafka

> models::Operation create_dbaas_service_kafka(name, create_dbaas_service_kafka_request)
Create a DBaaS Kafka service

Create a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_kafka_request** | [**CreateDbaasServiceKafkaRequest**](CreateDbaasServiceKafkaRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_mysql

> models::Operation create_dbaas_service_mysql(name, create_dbaas_service_mysql_request)
Create a DBaaS MySQL service

Create a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_mysql_request** | [**CreateDbaasServiceMysqlRequest**](CreateDbaasServiceMysqlRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_opensearch

> models::Operation create_dbaas_service_opensearch(name, create_dbaas_service_opensearch_request)
Create a DBaaS OpenSearch service

Create a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_opensearch_request** | [**CreateDbaasServiceOpensearchRequest**](CreateDbaasServiceOpensearchRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_pg

> models::Operation create_dbaas_service_pg(name, create_dbaas_service_pg_request)
Create a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_pg_request** | [**CreateDbaasServicePgRequest**](CreateDbaasServicePgRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_redis

> models::Operation create_dbaas_service_redis(name, create_dbaas_service_redis_request)
Create a DBaaS Redis service

Create a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_redis_request** | [**CreateDbaasServiceRedisRequest**](CreateDbaasServiceRedisRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_valkey

> models::Operation create_dbaas_service_valkey(name, create_dbaas_service_valkey_request)
Create a DBaaS Valkey service

Create a DBaaS Valkey service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_valkey_request** | [**CreateDbaasServiceValkeyRequest**](CreateDbaasServiceValkeyRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_task_migration_check

> models::Operation create_dbaas_task_migration_check(service, create_dbaas_task_migration_check_request)


Create a DBaaS task to check migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**create_dbaas_task_migration_check_request** | [**CreateDbaasTaskMigrationCheckRequest**](CreateDbaasTaskMigrationCheckRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_valkey_user

> models::Operation create_dbaas_valkey_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS Valkey user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_external_endpoint_datadog

> models::Operation delete_dbaas_external_endpoint_datadog(endpoint_id)


[BETA] Delete DataDog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_external_endpoint_elasticsearch

> models::Operation delete_dbaas_external_endpoint_elasticsearch(endpoint_id)


[BETA] Delete ElasticSearch logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_external_endpoint_opensearch

> models::Operation delete_dbaas_external_endpoint_opensearch(endpoint_id)


[BETA] Delete OpenSearch logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_external_endpoint_prometheus

> models::Operation delete_dbaas_external_endpoint_prometheus(endpoint_id)


[BETA] Delete Prometheus external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_external_endpoint_rsyslog

> models::Operation delete_dbaas_external_endpoint_rsyslog(endpoint_id)


[BETA] Delete RSyslog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_integration

> models::Operation delete_dbaas_integration(id)


[BETA] Delete a DBaaS Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_schema_registry_acl_config

> models::Operation delete_dbaas_kafka_schema_registry_acl_config(name, acl_id)
Delete a Kafka ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**acl_id** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_topic_acl_config

> models::Operation delete_dbaas_kafka_topic_acl_config(name, acl_id)
Delete a Kafka ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**acl_id** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_user

> models::Operation delete_dbaas_kafka_user(service_name, username)
Delete a DBaaS kafka user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_mysql_database

> models::Operation delete_dbaas_mysql_database(service_name, database_name)
Delete a DBaaS MySQL database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**database_name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_mysql_user

> models::Operation delete_dbaas_mysql_user(service_name, username)
Delete a DBaaS MySQL user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_opensearch_user

> models::Operation delete_dbaas_opensearch_user(service_name, username)
Delete a DBaaS OpenSearch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_pg_connection_pool

> models::Operation delete_dbaas_pg_connection_pool(service_name, connection_pool_name)
Delete a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**connection_pool_name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_pg_database

> models::Operation delete_dbaas_pg_database(service_name, database_name)
Delete a DBaaS Postgres database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**database_name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_postgres_user

> models::Operation delete_dbaas_postgres_user(service_name, username)
Delete a DBaaS Postgres user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_redis_user

> models::Operation delete_dbaas_redis_user(service_name, username)
Delete a DBaaS Redis user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service

> models::Operation delete_dbaas_service(name)
Delete a DBaaS service

Delete a DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_grafana

> models::Operation delete_dbaas_service_grafana(name)
Delete a Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_kafka

> models::Operation delete_dbaas_service_kafka(name)
Delete a Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_mysql

> models::Operation delete_dbaas_service_mysql(name)
Delete a MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_opensearch

> models::Operation delete_dbaas_service_opensearch(name)
Delete a OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_pg

> models::Operation delete_dbaas_service_pg(name)
Delete a Postgres service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_redis

> models::Operation delete_dbaas_service_redis(name)
Delete a Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_valkey

> models::Operation delete_dbaas_service_valkey(name)
Delete a Valkey service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_valkey_user

> models::Operation delete_dbaas_valkey_user(service_name, username)
Delete a DBaaS Valkey user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_dbaas_service_from_endpoint

> models::Operation detach_dbaas_service_from_endpoint(source_service_name, detach_dbaas_service_from_endpoint_request)


[BETA] Detach a DBaaS external integration from a service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_service_name** | **String** |  | [required] |
**detach_dbaas_service_from_endpoint_request** | [**DetachDbaasServiceFromEndpointRequest**](DetachDbaasServiceFromEndpointRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_dbaas_mysql_writes

> models::Operation enable_dbaas_mysql_writes(name)
Temporarily enable writes for MySQL services in read-only mode due to filled up storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_ca_certificate

> models::GetDbaasCaCertificate200Response get_dbaas_ca_certificate()
Get DBaaS CA Certificate

Returns a CA Certificate required to reach a DBaaS service through a TLS-protected connection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasCaCertificate200Response**](get_dbaas_ca_certificate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_endpoint_datadog

> models::DbaasExternalEndpointDatadogOutput get_dbaas_external_endpoint_datadog(endpoint_id)


[BETA] Get DataDog external endpoint settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasExternalEndpointDatadogOutput**](dbaas-external-endpoint-datadog-output.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_endpoint_elasticsearch

> models::DbaasEndpointElasticsearchOutput get_dbaas_external_endpoint_elasticsearch(endpoint_id)


[BETA] Get ElasticSearch Logs external integration endpoint settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasEndpointElasticsearchOutput**](dbaas-endpoint-elasticsearch-output.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_endpoint_opensearch

> models::DbaasEndpointOpensearchOutput get_dbaas_external_endpoint_opensearch(endpoint_id)


[BETA] Get OpenSearch Logs external integration endpoint settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasEndpointOpensearchOutput**](dbaas-endpoint-opensearch-output.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_endpoint_prometheus

> models::DbaasEndpointExternalPrometheusOutput get_dbaas_external_endpoint_prometheus(endpoint_id)


[BETA] Get Prometheus external integration endpoint settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasEndpointExternalPrometheusOutput**](dbaas-endpoint-external-prometheus-output.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_endpoint_rsyslog

> models::DbaasExternalEndpointRsyslogOutput get_dbaas_external_endpoint_rsyslog(endpoint_id)


[BETA] Get RSyslog external integration endpoint settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasExternalEndpointRsyslogOutput**](dbaas-external-endpoint-rsyslog-output.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_integration

> models::DbaasExternalIntegration get_dbaas_external_integration(integration_id)


[BETA] Get a DBaaS external integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasExternalIntegration**](dbaas-external-integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_external_integration_settings_datadog

> models::GetDbaasExternalIntegrationSettingsDatadog200Response get_dbaas_external_integration_settings_datadog(integration_id)


[BETA] Get Datadog integration settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetDbaasExternalIntegrationSettingsDatadog200Response**](get_dbaas_external_integration_settings_datadog_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_integration

> models::DbaasIntegration get_dbaas_integration(id)


[BETA] Get a DBaaS Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasIntegration**](dbaas-integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_kafka_acl_config

> models::DbaasKafkaAcls get_dbaas_kafka_acl_config(name)
Get DBaaS kafka ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasKafkaAcls**](dbaas-kafka-acls.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_migration_status

> models::DbaasMigrationStatus get_dbaas_migration_status(name)
Get a DBaaS migration status

Get a DBaaS migration status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasMigrationStatus**](dbaas-migration-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_opensearch_acl_config

> models::DbaasOpensearchAclConfig get_dbaas_opensearch_acl_config(name)
Get DBaaS OpenSearch ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasOpensearchAclConfig**](dbaas-opensearch-acl-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_grafana

> models::DbaasServiceGrafana get_dbaas_service_grafana(name)
Get a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceGrafana**](dbaas-service-grafana.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_kafka

> models::DbaasServiceKafka get_dbaas_service_kafka(name)
Get a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceKafka**](dbaas-service-kafka.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_logs

> models::DbaasServiceLogs get_dbaas_service_logs(service_name, get_dbaas_service_logs_request)
Get logs of DBaaS service

Get logs of DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**get_dbaas_service_logs_request** | [**GetDbaasServiceLogsRequest**](GetDbaasServiceLogsRequest.md) |  | [required] |

### Return type

[**models::DbaasServiceLogs**](dbaas-service-logs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_metrics

> models::GetDbaasServiceMetrics200Response get_dbaas_service_metrics(service_name, get_dbaas_service_metrics_request)
Get metrics of DBaaS service

Get metrics of DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**get_dbaas_service_metrics_request** | [**GetDbaasServiceMetricsRequest**](GetDbaasServiceMetricsRequest.md) |  | [required] |

### Return type

[**models::GetDbaasServiceMetrics200Response**](get_dbaas_service_metrics_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_mysql

> models::DbaasServiceMysql get_dbaas_service_mysql(name)
Get a DBaaS MySQL service

Get a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceMysql**](dbaas-service-mysql.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_opensearch

> models::DbaasServiceOpensearch get_dbaas_service_opensearch(name)
Get a DBaaS OpenSearch service

Get a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceOpensearch**](dbaas-service-opensearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_pg

> models::DbaasServicePg get_dbaas_service_pg(name)
Get a DBaaS PostgreSQL service

Get a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServicePg**](dbaas-service-pg.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_redis

> models::DbaasServiceRedis get_dbaas_service_redis(name)
Get a DBaaS Redis service

Get a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceRedis**](dbaas-service-redis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_type

> models::DbaasServiceType get_dbaas_service_type(service_type_name)
Get a DBaaS service type

Get a DBaaS service type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_type_name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceType**](dbaas-service-type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_valkey

> models::DbaasServiceValkey get_dbaas_service_valkey(name)


Get a DBaaS Valkey service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DbaasServiceValkey**](dbaas-service-valkey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_grafana

> models::GetDbaasSettingsGrafana200Response get_dbaas_settings_grafana()
Get DBaaS Grafana settings

Get DBaaS Grafana settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsGrafana200Response**](get_dbaas_settings_grafana_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_kafka

> models::GetDbaasSettingsKafka200Response get_dbaas_settings_kafka()
Get DBaaS Kafka settings

Get DBaaS Kafka settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsKafka200Response**](get_dbaas_settings_kafka_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_mysql

> models::GetDbaasSettingsMysql200Response get_dbaas_settings_mysql()
Get DBaaS MySQL settings

Get DBaaS MySQL settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsMysql200Response**](get_dbaas_settings_mysql_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_opensearch

> models::GetDbaasSettingsOpensearch200Response get_dbaas_settings_opensearch()
Get DBaaS OpenSearch settings

Get DBaaS OpenSearch settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsOpensearch200Response**](get_dbaas_settings_opensearch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_pg

> models::GetDbaasSettingsPg200Response get_dbaas_settings_pg()
Get DBaaS PostgreSQL settings

Get DBaaS PostgreSQL settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsPg200Response**](get_dbaas_settings_pg_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_redis

> models::GetDbaasSettingsRedis200Response get_dbaas_settings_redis()
Get DBaaS Redis settings

Returns the default settings for Redis.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsRedis200Response**](get_dbaas_settings_redis_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_valkey

> models::GetDbaasSettingsValkey200Response get_dbaas_settings_valkey()
Get DBaaS Valkey settings

Returns the default settings for Valkey.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDbaasSettingsValkey200Response**](get_dbaas_settings_valkey_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_task

> models::DbaasTask get_dbaas_task(service, id)
Get a DBaaS task

Get a DBaaS task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DbaasTask**](dbaas-task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_external_endpoint_types

> models::ListDbaasExternalEndpointTypes200Response list_dbaas_external_endpoint_types()


[BETA] List available external endpoint types and their schemas for DBaaS external integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDbaasExternalEndpointTypes200Response**](list_dbaas_external_endpoint_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_external_endpoints

> models::ListDbaasExternalEndpoints200Response list_dbaas_external_endpoints()


[BETA] List available external endpoints for integrations

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDbaasExternalEndpoints200Response**](list_dbaas_external_endpoints_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_external_integrations

> models::ListDbaasExternalIntegrations200Response list_dbaas_external_integrations(service_name)


[BETA] List all DBaaS connections between services and external endpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |

### Return type

[**models::ListDbaasExternalIntegrations200Response**](list_dbaas_external_integrations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_integration_settings

> models::ListDbaasIntegrationSettings200Response list_dbaas_integration_settings(integration_type, source_type, dest_type)


[BETA] Get DBaaS integration settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_type** | **String** |  | [required] |
**source_type** | **String** |  | [required] |
**dest_type** | **String** |  | [required] |

### Return type

[**models::ListDbaasIntegrationSettings200Response**](list_dbaas_integration_settings_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_integration_types

> models::ListDbaasIntegrationTypes200Response list_dbaas_integration_types()


[BETA] Get DBaaS integration types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDbaasIntegrationTypes200Response**](list_dbaas_integration_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_service_types

> models::ListDbaasServiceTypes200Response list_dbaas_service_types()
DBaaS Service Types

List available service types for DBaaS

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDbaasServiceTypes200Response**](list_dbaas_service_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_services

> models::ListDbaasServices200Response list_dbaas_services()
List DBaaS services

List DBaaS services

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDbaasServices200Response**](list_dbaas_services_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_grafana_user_password

> models::Operation reset_dbaas_grafana_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS Grafana user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_kafka_user_password

> models::Operation reset_dbaas_kafka_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS Kafka user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_mysql_user_password

> models::Operation reset_dbaas_mysql_user_password(service_name, username, reset_dbaas_mysql_user_password_request)
Reset the credentials of a DBaaS mysql user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_mysql_user_password_request** | [**ResetDbaasMysqlUserPasswordRequest**](ResetDbaasMysqlUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_opensearch_user_password

> models::Operation reset_dbaas_opensearch_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS OpenSearch user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_postgres_user_password

> models::Operation reset_dbaas_postgres_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS Postgres user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_redis_user_password

> models::Operation reset_dbaas_redis_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS Redis user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_valkey_user_password

> models::Operation reset_dbaas_valkey_user_password(service_name, username, reset_dbaas_valkey_user_password_request)
Reset the credentials of a DBaaS Valkey user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_valkey_user_password_request** | [**ResetDbaasValkeyUserPasswordRequest**](ResetDbaasValkeyUserPasswordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_grafana_user_password

> models::DbaasUserGrafanaSecrets reveal_dbaas_grafana_user_password(service_name, username)
Reveal the secrets of a DBaaS Grafana user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserGrafanaSecrets**](dbaas-user-grafana-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_kafka_connect_password

> models::DbaasUserKafkaConnectSecrets reveal_dbaas_kafka_connect_password(service_name)
Reveal the secrets for DBaaS Kafka Connect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |

### Return type

[**models::DbaasUserKafkaConnectSecrets**](dbaas-user-kafka-connect-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_kafka_user_password

> models::DbaasUserKafkaSecrets reveal_dbaas_kafka_user_password(service_name, username)
Reveal the secrets of a DBaaS Kafka user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserKafkaSecrets**](dbaas-user-kafka-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_mysql_user_password

> models::DbaasUserMysqlSecrets reveal_dbaas_mysql_user_password(service_name, username)
Reveal the secrets of a DBaaS MySQL user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserMysqlSecrets**](dbaas-user-mysql-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_opensearch_user_password

> models::DbaasUserOpensearchSecrets reveal_dbaas_opensearch_user_password(service_name, username)
Reveal the secrets of a DBaaS OpenSearch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserOpensearchSecrets**](dbaas-user-opensearch-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_postgres_user_password

> models::DbaasUserPostgresSecrets reveal_dbaas_postgres_user_password(service_name, username)
Reveal the secrets of a DBaaS Postgres user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserPostgresSecrets**](dbaas-user-postgres-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_redis_user_password

> models::DbaasUserRedisSecrets reveal_dbaas_redis_user_password(service_name, username)
Reveal the secrets of a DBaaS Redis user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserRedisSecrets**](dbaas-user-redis-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_dbaas_valkey_user_password

> models::DbaasUserValkeySecrets reveal_dbaas_valkey_user_password(service_name, username)
Reveal the secrets of a DBaaS Valkey user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::DbaasUserValkeySecrets**](dbaas-user-valkey-secrets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_grafana_maintenance

> models::Operation start_dbaas_grafana_maintenance(name)
Initiate Grafana maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_kafka_maintenance

> models::Operation start_dbaas_kafka_maintenance(name)
Initiate Kafka maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_mysql_maintenance

> models::Operation start_dbaas_mysql_maintenance(name)
Initiate MySQL maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_opensearch_maintenance

> models::Operation start_dbaas_opensearch_maintenance(name)
Initiate OpenSearch maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_pg_maintenance

> models::Operation start_dbaas_pg_maintenance(name)
Initiate PostgreSQL maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_redis_maintenance

> models::Operation start_dbaas_redis_maintenance(name)
Initiate Redis maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_redis_to_valkey_upgrade

> models::Operation start_dbaas_redis_to_valkey_upgrade(name)
Initiate Redis upgrade to Valkey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_valkey_maintenance

> models::Operation start_dbaas_valkey_maintenance(name)
Initiate Valkey maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_mysql_migration

> models::Operation stop_dbaas_mysql_migration(name)
Stop a DBaaS MySQL migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_pg_migration

> models::Operation stop_dbaas_pg_migration(name)
Stop a DBaaS PostgreSQL migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_redis_migration

> models::Operation stop_dbaas_redis_migration(name)
Stop a DBaaS Redis migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_valkey_migration

> models::Operation stop_dbaas_valkey_migration(name)
Stop a DBaaS Valkey migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_endpoint_datadog

> models::Operation update_dbaas_external_endpoint_datadog(endpoint_id, dbaas_endpoint_datadog_input_update)


[BETA] Update DataDog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |
**dbaas_endpoint_datadog_input_update** | [**DbaasEndpointDatadogInputUpdate**](DbaasEndpointDatadogInputUpdate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_endpoint_elasticsearch

> models::Operation update_dbaas_external_endpoint_elasticsearch(endpoint_id, dbaas_endpoint_elasticsearch_input_update)


[BETA] Update ElasticSearch Logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |
**dbaas_endpoint_elasticsearch_input_update** | [**DbaasEndpointElasticsearchInputUpdate**](DbaasEndpointElasticsearchInputUpdate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_endpoint_opensearch

> models::Operation update_dbaas_external_endpoint_opensearch(endpoint_id, dbaas_endpoint_opensearch_input_update)


[BETA] Update OpenSearch Logs external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |
**dbaas_endpoint_opensearch_input_update** | [**DbaasEndpointOpensearchInputUpdate**](DbaasEndpointOpensearchInputUpdate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_endpoint_prometheus

> models::Operation update_dbaas_external_endpoint_prometheus(endpoint_id, dbaas_endpoint_prometheus_payload)


[BETA] Update Prometheus external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |
**dbaas_endpoint_prometheus_payload** | [**DbaasEndpointPrometheusPayload**](DbaasEndpointPrometheusPayload.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_endpoint_rsyslog

> models::Operation update_dbaas_external_endpoint_rsyslog(endpoint_id, dbaas_endpoint_rsyslog_input_update)


[BETA] Update RSyslog external integration endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **uuid::Uuid** |  | [required] |
**dbaas_endpoint_rsyslog_input_update** | [**DbaasEndpointRsyslogInputUpdate**](DbaasEndpointRsyslogInputUpdate.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_external_integration_settings_datadog

> models::Operation update_dbaas_external_integration_settings_datadog(integration_id, get_dbaas_external_integration_settings_datadog200_response)


[BETA] Manage Datadog integration settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **uuid::Uuid** |  | [required] |
**get_dbaas_external_integration_settings_datadog200_response** | [**GetDbaasExternalIntegrationSettingsDatadog200Response**](GetDbaasExternalIntegrationSettingsDatadog200Response.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_integration

> models::Operation update_dbaas_integration(id, update_dbaas_integration_request)


[BETA] Update a existing DBaaS integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_dbaas_integration_request** | [**UpdateDbaasIntegrationRequest**](UpdateDbaasIntegrationRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_opensearch_acl_config

> models::Operation update_dbaas_opensearch_acl_config(name, dbaas_opensearch_acl_config)
Create a DBaaS OpenSearch ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_opensearch_acl_config** | [**DbaasOpensearchAclConfig**](DbaasOpensearchAclConfig.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_pg_connection_pool

> models::Operation update_dbaas_pg_connection_pool(service_name, connection_pool_name, update_dbaas_pg_connection_pool_request)
Update a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**connection_pool_name** | **String** |  | [required] |
**update_dbaas_pg_connection_pool_request** | [**UpdateDbaasPgConnectionPoolRequest**](UpdateDbaasPgConnectionPoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_postgres_allow_replication

> models::DbaasPostgresUsers update_dbaas_postgres_allow_replication(service_name, username, update_dbaas_postgres_allow_replication_request)
Update access control for one service user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**update_dbaas_postgres_allow_replication_request** | [**UpdateDbaasPostgresAllowReplicationRequest**](UpdateDbaasPostgresAllowReplicationRequest.md) |  | [required] |

### Return type

[**models::DbaasPostgresUsers**](dbaas-postgres-users.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_grafana

> models::Operation update_dbaas_service_grafana(name, update_dbaas_service_grafana_request)
Update a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_grafana_request** | [**UpdateDbaasServiceGrafanaRequest**](UpdateDbaasServiceGrafanaRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_kafka

> models::Operation update_dbaas_service_kafka(name, update_dbaas_service_kafka_request)
Update a DBaaS Kafka service

Update a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_kafka_request** | [**UpdateDbaasServiceKafkaRequest**](UpdateDbaasServiceKafkaRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_mysql

> models::Operation update_dbaas_service_mysql(name, update_dbaas_service_mysql_request)
Update a DBaaS MySQL service

Update a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_mysql_request** | [**UpdateDbaasServiceMysqlRequest**](UpdateDbaasServiceMysqlRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_opensearch

> models::Operation update_dbaas_service_opensearch(name, update_dbaas_service_opensearch_request)
Update a DBaaS OpenSearch service

Update a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_opensearch_request** | [**UpdateDbaasServiceOpensearchRequest**](UpdateDbaasServiceOpensearchRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_pg

> models::Operation update_dbaas_service_pg(name, update_dbaas_service_pg_request)
Update a DBaaS PostgreSQL service

Update a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_pg_request** | [**UpdateDbaasServicePgRequest**](UpdateDbaasServicePgRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_redis

> models::Operation update_dbaas_service_redis(name, update_dbaas_service_redis_request)
Update a DBaaS Redis service

Update a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_redis_request** | [**UpdateDbaasServiceRedisRequest**](UpdateDbaasServiceRedisRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_valkey

> models::Operation update_dbaas_service_valkey(name, update_dbaas_service_valkey_request)


Update a DBaaS Valkey service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_valkey_request** | [**UpdateDbaasServiceValkeyRequest**](UpdateDbaasServiceValkeyRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

