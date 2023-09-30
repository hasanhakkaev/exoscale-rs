# \DbaasApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dbaas_integration**](DbaasApi.md#create_dbaas_integration) | **POST** /dbaas-integration | Create a new DBaaS integration between two services
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
[**create_dbaas_service_grafana**](DbaasApi.md#create_dbaas_service_grafana) | **POST** /dbaas-grafana/{name} | 
[**create_dbaas_service_kafka**](DbaasApi.md#create_dbaas_service_kafka) | **POST** /dbaas-kafka/{name} | Create a DBaaS Kafka service
[**create_dbaas_service_mysql**](DbaasApi.md#create_dbaas_service_mysql) | **POST** /dbaas-mysql/{name} | Create a DBaaS MySQL service
[**create_dbaas_service_opensearch**](DbaasApi.md#create_dbaas_service_opensearch) | **POST** /dbaas-opensearch/{name} | Create a DBaaS OpenSearch service
[**create_dbaas_service_pg**](DbaasApi.md#create_dbaas_service_pg) | **POST** /dbaas-postgres/{name} | Create a DBaaS PostgreSQL service
[**create_dbaas_service_redis**](DbaasApi.md#create_dbaas_service_redis) | **POST** /dbaas-redis/{name} | Create a DBaaS Redis service
[**create_dbaas_task_migration_check**](DbaasApi.md#create_dbaas_task_migration_check) | **POST** /dbaas-task-migration-check/{service} | 
[**delete_dbaas_integration**](DbaasApi.md#delete_dbaas_integration) | **DELETE** /dbaas-integration/{id} | Delete a DBaaS Integration
[**delete_dbaas_kafka_schema_registry_acl_config**](DbaasApi.md#delete_dbaas_kafka_schema_registry_acl_config) | **DELETE** /dbaas-kafka/{name}/schema-registry/acl-config/{acl_id} | Delete a Kafka ACL entry
[**delete_dbaas_kafka_topic_acl_config**](DbaasApi.md#delete_dbaas_kafka_topic_acl_config) | **DELETE** /dbaas-kafka/{name}/topic/acl-config/{acl_id} | Delete a Kafka ACL entry
[**delete_dbaas_kafka_user**](DbaasApi.md#delete_dbaas_kafka_user) | **DELETE** /dbaas-kafka/{service_name}/user/{username} | Delete a DBaaS kafka user
[**delete_dbaas_mysql_database**](DbaasApi.md#delete_dbaas_mysql_database) | **DELETE** /dbaas-mysql/{service_name}/database/{database_name} | Delete a DBaaS MySQL database
[**delete_dbaas_mysql_user**](DbaasApi.md#delete_dbaas_mysql_user) | **DELETE** /dbaas-mysql/{service_name}/user/{username} | Delete a DBaaS MySQL user
[**delete_dbaas_opensearch_user**](DbaasApi.md#delete_dbaas_opensearch_user) | **DELETE** /dbaas-opensearch/{service_name}/user/{username} | Delete a DBaaS OpenSearch user
[**delete_dbaas_pg_connection_pool**](DbaasApi.md#delete_dbaas_pg_connection_pool) | **DELETE** /dbaas-postgres/{service_name}/connection-pool/{connection_pool_name} | Delete a DBaaS PostgreSQL connection pool
[**delete_dbaas_pg_database**](DbaasApi.md#delete_dbaas_pg_database) | **DELETE** /dbaas-postgres/{service_name}/database/{database_name} | Delete a DBaaS Postgres database
[**delete_dbaas_postgres_user**](DbaasApi.md#delete_dbaas_postgres_user) | **DELETE** /dbaas-postgres/{service_name}/user/{username} | Delete a DBaaS Postgres user
[**delete_dbaas_service**](DbaasApi.md#delete_dbaas_service) | **DELETE** /dbaas-service/{name} | Delete a DBaaS service
[**delete_dbaas_service_grafana**](DbaasApi.md#delete_dbaas_service_grafana) | **DELETE** /dbaas-grafana/{name} | Delete a Grafana service
[**delete_dbaas_service_kafka**](DbaasApi.md#delete_dbaas_service_kafka) | **DELETE** /dbaas-kafka/{name} | Delete a Kafka service
[**delete_dbaas_service_mysql**](DbaasApi.md#delete_dbaas_service_mysql) | **DELETE** /dbaas-mysql/{name} | Delete a MySQL service
[**delete_dbaas_service_opensearch**](DbaasApi.md#delete_dbaas_service_opensearch) | **DELETE** /dbaas-opensearch/{name} | Delete a OpenSearch service
[**delete_dbaas_service_pg**](DbaasApi.md#delete_dbaas_service_pg) | **DELETE** /dbaas-postgres/{name} | Delete a Postgres service
[**delete_dbaas_service_redis**](DbaasApi.md#delete_dbaas_service_redis) | **DELETE** /dbaas-redis/{name} | Delete a Redis service
[**get_dbaas_ca_certificate**](DbaasApi.md#get_dbaas_ca_certificate) | **GET** /dbaas-ca-certificate | Get DBaaS CA Certificate
[**get_dbaas_integration**](DbaasApi.md#get_dbaas_integration) | **GET** /dbaas-integration/{id} | Get a DBaaS Integration
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
[**get_dbaas_service_type**](DbaasApi.md#get_dbaas_service_type) | **GET** /dbaas-service-type/{service-type_name} | Get a DBaaS service type
[**get_dbaas_settings_grafana**](DbaasApi.md#get_dbaas_settings_grafana) | **GET** /dbaas-settings-grafana | Get DBaaS Grafana settings
[**get_dbaas_settings_kafka**](DbaasApi.md#get_dbaas_settings_kafka) | **GET** /dbaas-settings-kafka | Get DBaaS Kafka settings
[**get_dbaas_settings_mysql**](DbaasApi.md#get_dbaas_settings_mysql) | **GET** /dbaas-settings-mysql | Get DBaaS MySQL settings
[**get_dbaas_settings_opensearch**](DbaasApi.md#get_dbaas_settings_opensearch) | **GET** /dbaas-settings-opensearch | Get DBaaS OpenSearch settings
[**get_dbaas_settings_pg**](DbaasApi.md#get_dbaas_settings_pg) | **GET** /dbaas-settings-pg | Get DBaaS PostgreSQL settings
[**get_dbaas_settings_redis**](DbaasApi.md#get_dbaas_settings_redis) | **GET** /dbaas-settings-redis | Get DBaaS Redis settings
[**get_dbaas_task**](DbaasApi.md#get_dbaas_task) | **GET** /dbaas-task/{service}/{id} | Get a DBaaS task
[**list_dbaas_integration_settings**](DbaasApi.md#list_dbaas_integration_settings) | **GET** /dbaas-integration-settings/{integration-type}/{source-type}/{dest-type} | Get DBaaS integration settings
[**list_dbaas_integration_types**](DbaasApi.md#list_dbaas_integration_types) | **GET** /dbaas-integration-types | Get DBaaS integration types
[**list_dbaas_service_types**](DbaasApi.md#list_dbaas_service_types) | **GET** /dbaas-service-type | DBaaS Service Types
[**list_dbaas_services**](DbaasApi.md#list_dbaas_services) | **GET** /dbaas-service | List DBaaS services
[**reset_dbaas_kafka_user_password**](DbaasApi.md#reset_dbaas_kafka_user_password) | **PUT** /dbaas-kafka/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Kafka user
[**reset_dbaas_mysql_user_password**](DbaasApi.md#reset_dbaas_mysql_user_password) | **PUT** /dbaas-mysql/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS mysql user
[**reset_dbaas_opensearch_user_password**](DbaasApi.md#reset_dbaas_opensearch_user_password) | **PUT** /dbaas-opensearch/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS OpenSearch user
[**reset_dbaas_postgres_user_password**](DbaasApi.md#reset_dbaas_postgres_user_password) | **PUT** /dbaas-postgres/{service_name}/user/{username}/password/reset | Reset the credentials of a DBaaS Postgres user
[**start_dbaas_grafana_maintenance**](DbaasApi.md#start_dbaas_grafana_maintenance) | **PUT** /dbaas-grafana/{name}/maintenance/start | Initiate Grafana maintenance update
[**start_dbaas_kafka_maintenance**](DbaasApi.md#start_dbaas_kafka_maintenance) | **PUT** /dbaas-kafka/{name}/maintenance/start | Initiate Kafka maintenance update
[**start_dbaas_mysql_maintenance**](DbaasApi.md#start_dbaas_mysql_maintenance) | **PUT** /dbaas-mysql/{name}/maintenance/start | Initiate MySQL maintenance update
[**start_dbaas_opensearch_maintenance**](DbaasApi.md#start_dbaas_opensearch_maintenance) | **PUT** /dbaas-opensearch/{name}/maintenance/start | Initiate OpenSearch maintenance update
[**start_dbaas_pg_maintenance**](DbaasApi.md#start_dbaas_pg_maintenance) | **PUT** /dbaas-postgres/{name}/maintenance/start | Initiate PostgreSQL maintenance update
[**start_dbaas_redis_maintenance**](DbaasApi.md#start_dbaas_redis_maintenance) | **PUT** /dbaas-redis/{name}/maintenance/start | Initiate Redis maintenance update
[**stop_dbaas_mysql_migration**](DbaasApi.md#stop_dbaas_mysql_migration) | **POST** /dbaas-mysql/{name}/migration/stop | Stop a DBaaS MySQL migration
[**stop_dbaas_pg_migration**](DbaasApi.md#stop_dbaas_pg_migration) | **POST** /dbaas-postgres/{name}/migration/stop | Stop a DBaaS PostgreSQL migration
[**stop_dbaas_redis_migration**](DbaasApi.md#stop_dbaas_redis_migration) | **POST** /dbaas-redis/{name}/migration/stop | Stop a DBaaS Redis migration
[**update_dbaas_integration**](DbaasApi.md#update_dbaas_integration) | **PUT** /dbaas-integration/{id} | Update a existing DBaaS integration
[**update_dbaas_opensearch_acl_config**](DbaasApi.md#update_dbaas_opensearch_acl_config) | **PUT** /dbaas-opensearch/{name}/acl-config | Create a DBaaS OpenSearch ACL configuration
[**update_dbaas_pg_connection_pool**](DbaasApi.md#update_dbaas_pg_connection_pool) | **PUT** /dbaas-postgres/{service_name}/connection-pool/{connection_pool_name} | Update a DBaaS PostgreSQL connection pool
[**update_dbaas_postgres_allow_replication**](DbaasApi.md#update_dbaas_postgres_allow_replication) | **PUT** /dbaas-postgres/{service_name}/user/{username}/allow-replication | Update access control for one service user
[**update_dbaas_service_grafana**](DbaasApi.md#update_dbaas_service_grafana) | **PUT** /dbaas-grafana/{name} | Update a DBaaS Grafana service
[**update_dbaas_service_kafka**](DbaasApi.md#update_dbaas_service_kafka) | **PUT** /dbaas-kafka/{name} | Update a DBaaS Kafka service
[**update_dbaas_service_mysql**](DbaasApi.md#update_dbaas_service_mysql) | **PUT** /dbaas-mysql/{name} | Update a DBaaS MySQL service
[**update_dbaas_service_opensearch**](DbaasApi.md#update_dbaas_service_opensearch) | **PUT** /dbaas-opensearch/{name} | Update a DBaaS OpenSearch service
[**update_dbaas_service_pg**](DbaasApi.md#update_dbaas_service_pg) | **PUT** /dbaas-postgres/{name} | Update a DBaaS PostgreSQL service
[**update_dbaas_service_redis**](DbaasApi.md#update_dbaas_service_redis) | **PUT** /dbaas-redis/{name} | Update a DBaaS Redis service



## create_dbaas_integration

> crate::models::Operation create_dbaas_integration(create_dbaas_integration_request)
Create a new DBaaS integration between two services

Create a new DBaaS integration between two services

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dbaas_integration_request** | [**CreateDbaasIntegrationRequest**](CreateDbaasIntegrationRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_schema_registry_acl_config

> crate::models::Operation create_dbaas_kafka_schema_registry_acl_config(name, dbaas_kafka_schema_registry_acl_entry)
Add a Kafka Schema Registry ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_kafka_schema_registry_acl_entry** | [**DbaasKafkaSchemaRegistryAclEntry**](DbaasKafkaSchemaRegistryAclEntry.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_topic_acl_config

> crate::models::Operation create_dbaas_kafka_topic_acl_config(name, dbaas_kafka_topic_acl_entry)
Add a Kafka topic ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_kafka_topic_acl_entry** | [**DbaasKafkaTopicAclEntry**](DbaasKafkaTopicAclEntry.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_kafka_user

> crate::models::Operation create_dbaas_kafka_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS Kafka user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_mysql_database

> crate::models::Operation create_dbaas_mysql_database(service_name, create_dbaas_mysql_database_request)
Create a DBaaS MySQL database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_mysql_database_request** | [**CreateDbaasMysqlDatabaseRequest**](CreateDbaasMysqlDatabaseRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_mysql_user

> crate::models::Operation create_dbaas_mysql_user(service_name, create_dbaas_mysql_user_request)
Create a DBaaS MySQL user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_mysql_user_request** | [**CreateDbaasMysqlUserRequest**](CreateDbaasMysqlUserRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_opensearch_user

> crate::models::Operation create_dbaas_opensearch_user(service_name, create_dbaas_kafka_user_request)
Create a DBaaS OpenSearch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_kafka_user_request** | [**CreateDbaasKafkaUserRequest**](CreateDbaasKafkaUserRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_connection_pool

> crate::models::Operation create_dbaas_pg_connection_pool(service_name, create_dbaas_pg_connection_pool_request)
Create a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_pg_connection_pool_request** | [**CreateDbaasPgConnectionPoolRequest**](CreateDbaasPgConnectionPoolRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_database

> crate::models::Operation create_dbaas_pg_database(service_name, create_dbaas_pg_database_request)
Create a DBaaS Postgres database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_pg_database_request** | [**CreateDbaasPgDatabaseRequest**](CreateDbaasPgDatabaseRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_pg_upgrade_check

> crate::models::DbaasTask create_dbaas_pg_upgrade_check(service, create_dbaas_pg_upgrade_check_request)


Check whether you can upgrade Postgres service to a newer version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**create_dbaas_pg_upgrade_check_request** | [**CreateDbaasPgUpgradeCheckRequest**](CreateDbaasPgUpgradeCheckRequest.md) |  | [required] |

### Return type

[**crate::models::DbaasTask**](dbaas-task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_postgres_user

> crate::models::Operation create_dbaas_postgres_user(service_name, create_dbaas_postgres_user_request)
Create a DBaaS Postgres user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**create_dbaas_postgres_user_request** | [**CreateDbaasPostgresUserRequest**](CreateDbaasPostgresUserRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_grafana

> crate::models::Operation create_dbaas_service_grafana(name, create_dbaas_service_grafana_request)


Create a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_grafana_request** | [**CreateDbaasServiceGrafanaRequest**](CreateDbaasServiceGrafanaRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_kafka

> crate::models::Operation create_dbaas_service_kafka(name, create_dbaas_service_kafka_request)
Create a DBaaS Kafka service

Create a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_kafka_request** | [**CreateDbaasServiceKafkaRequest**](CreateDbaasServiceKafkaRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_mysql

> crate::models::Operation create_dbaas_service_mysql(name, create_dbaas_service_mysql_request)
Create a DBaaS MySQL service

Create a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_mysql_request** | [**CreateDbaasServiceMysqlRequest**](CreateDbaasServiceMysqlRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_opensearch

> crate::models::Operation create_dbaas_service_opensearch(name, create_dbaas_service_opensearch_request)
Create a DBaaS OpenSearch service

Create a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_opensearch_request** | [**CreateDbaasServiceOpensearchRequest**](CreateDbaasServiceOpensearchRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_pg

> crate::models::Operation create_dbaas_service_pg(name, create_dbaas_service_pg_request)
Create a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_pg_request** | [**CreateDbaasServicePgRequest**](CreateDbaasServicePgRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_service_redis

> crate::models::Operation create_dbaas_service_redis(name, create_dbaas_service_redis_request)
Create a DBaaS Redis service

Create a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**create_dbaas_service_redis_request** | [**CreateDbaasServiceRedisRequest**](CreateDbaasServiceRedisRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dbaas_task_migration_check

> crate::models::Operation create_dbaas_task_migration_check(service, create_dbaas_task_migration_check_request)


Create a DBaaS task to check migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**create_dbaas_task_migration_check_request** | [**CreateDbaasTaskMigrationCheckRequest**](CreateDbaasTaskMigrationCheckRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_integration

> crate::models::Operation delete_dbaas_integration(id)
Delete a DBaaS Integration

Delete a DBaaS Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_schema_registry_acl_config

> crate::models::Operation delete_dbaas_kafka_schema_registry_acl_config(name, acl_id)
Delete a Kafka ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**acl_id** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_topic_acl_config

> crate::models::Operation delete_dbaas_kafka_topic_acl_config(name, acl_id)
Delete a Kafka ACL entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**acl_id** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_kafka_user

> crate::models::Operation delete_dbaas_kafka_user(service_name, username)
Delete a DBaaS kafka user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_mysql_database

> crate::models::Operation delete_dbaas_mysql_database(service_name, database_name)
Delete a DBaaS MySQL database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**database_name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_mysql_user

> crate::models::Operation delete_dbaas_mysql_user(service_name, username)
Delete a DBaaS MySQL user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_opensearch_user

> crate::models::Operation delete_dbaas_opensearch_user(service_name, username)
Delete a DBaaS OpenSearch user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_pg_connection_pool

> crate::models::Operation delete_dbaas_pg_connection_pool(service_name, connection_pool_name)
Delete a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**connection_pool_name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_pg_database

> crate::models::Operation delete_dbaas_pg_database(service_name, database_name)
Delete a DBaaS Postgres database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**database_name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_postgres_user

> crate::models::Operation delete_dbaas_postgres_user(service_name, username)
Delete a DBaaS Postgres user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service

> crate::models::Operation delete_dbaas_service(name)
Delete a DBaaS service

Delete a DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_grafana

> crate::models::Operation delete_dbaas_service_grafana(name)
Delete a Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_kafka

> crate::models::Operation delete_dbaas_service_kafka(name)
Delete a Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_mysql

> crate::models::Operation delete_dbaas_service_mysql(name)
Delete a MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_opensearch

> crate::models::Operation delete_dbaas_service_opensearch(name)
Delete a OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_pg

> crate::models::Operation delete_dbaas_service_pg(name)
Delete a Postgres service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dbaas_service_redis

> crate::models::Operation delete_dbaas_service_redis(name)
Delete a Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_ca_certificate

> crate::models::GetDbaasCaCertificate200Response get_dbaas_ca_certificate()
Get DBaaS CA Certificate

Returns a CA Certificate required to reach a DBaaS service through a TLS-protected connection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasCaCertificate200Response**](get_dbaas_ca_certificate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_integration

> crate::models::DbaasIntegration get_dbaas_integration(id)
Get a DBaaS Integration

Get a DBaaS Integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::DbaasIntegration**](dbaas-integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_kafka_acl_config

> crate::models::DbaasKafkaAcls get_dbaas_kafka_acl_config(name)
Get DBaaS kafka ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasKafkaAcls**](dbaas-kafka-acls.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_migration_status

> crate::models::DbaasMigrationStatus get_dbaas_migration_status(name)
Get a DBaaS migration status

Get a DBaaS migration status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasMigrationStatus**](dbaas-migration-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_opensearch_acl_config

> crate::models::DbaasOpensearchAclConfig get_dbaas_opensearch_acl_config(name)
Get DBaaS OpenSearch ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasOpensearchAclConfig**](dbaas-opensearch-acl-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_grafana

> crate::models::DbaasServiceGrafana get_dbaas_service_grafana(name)
Get a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceGrafana**](dbaas-service-grafana.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_kafka

> crate::models::DbaasServiceKafka get_dbaas_service_kafka(name)
Get a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceKafka**](dbaas-service-kafka.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_logs

> crate::models::DbaasServiceLogs get_dbaas_service_logs(service_name, get_dbaas_service_logs_request)
Get logs of DBaaS service

Get logs of DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**get_dbaas_service_logs_request** | [**GetDbaasServiceLogsRequest**](GetDbaasServiceLogsRequest.md) |  | [required] |

### Return type

[**crate::models::DbaasServiceLogs**](dbaas-service-logs.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_metrics

> crate::models::GetDbaasServiceMetrics200Response get_dbaas_service_metrics(service_name, get_dbaas_service_metrics_request)
Get metrics of DBaaS service

Get metrics of DBaaS service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**get_dbaas_service_metrics_request** | [**GetDbaasServiceMetricsRequest**](GetDbaasServiceMetricsRequest.md) |  | [required] |

### Return type

[**crate::models::GetDbaasServiceMetrics200Response**](get_dbaas_service_metrics_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_mysql

> crate::models::DbaasServiceMysql get_dbaas_service_mysql(name)
Get a DBaaS MySQL service

Get a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceMysql**](dbaas-service-mysql.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_opensearch

> crate::models::DbaasServiceOpensearch get_dbaas_service_opensearch(name)
Get a DBaaS OpenSearch service

Get a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceOpensearch**](dbaas-service-opensearch.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_pg

> crate::models::DbaasServicePg get_dbaas_service_pg(name)
Get a DBaaS PostgreSQL service

Get a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServicePg**](dbaas-service-pg.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_redis

> crate::models::DbaasServiceRedis get_dbaas_service_redis(name)
Get a DBaaS Redis service

Get a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceRedis**](dbaas-service-redis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_service_type

> crate::models::DbaasServiceType get_dbaas_service_type(service_type_name)
Get a DBaaS service type

Get a DBaaS service type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_type_name** | **String** |  | [required] |

### Return type

[**crate::models::DbaasServiceType**](dbaas-service-type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_grafana

> crate::models::GetDbaasSettingsGrafana200Response get_dbaas_settings_grafana()
Get DBaaS Grafana settings

Get DBaaS Grafana settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsGrafana200Response**](get_dbaas_settings_grafana_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_kafka

> crate::models::GetDbaasSettingsKafka200Response get_dbaas_settings_kafka()
Get DBaaS Kafka settings

Get DBaaS Kafka settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsKafka200Response**](get_dbaas_settings_kafka_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_mysql

> crate::models::GetDbaasSettingsMysql200Response get_dbaas_settings_mysql()
Get DBaaS MySQL settings

Get DBaaS MySQL settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsMysql200Response**](get_dbaas_settings_mysql_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_opensearch

> crate::models::GetDbaasSettingsOpensearch200Response get_dbaas_settings_opensearch()
Get DBaaS OpenSearch settings

Get DBaaS OpenSearch settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsOpensearch200Response**](get_dbaas_settings_opensearch_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_pg

> crate::models::GetDbaasSettingsPg200Response get_dbaas_settings_pg()
Get DBaaS PostgreSQL settings

Get DBaaS PostgreSQL settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsPg200Response**](get_dbaas_settings_pg_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_settings_redis

> crate::models::GetDbaasSettingsRedis200Response get_dbaas_settings_redis()
Get DBaaS Redis settings

Returns the default settings for Redis.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetDbaasSettingsRedis200Response**](get_dbaas_settings_redis_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dbaas_task

> crate::models::DbaasTask get_dbaas_task(service, id)
Get a DBaaS task

Get a DBaaS task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service** | **String** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::DbaasTask**](dbaas-task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_integration_settings

> crate::models::ListDbaasIntegrationSettings200Response list_dbaas_integration_settings(integration_type, source_type, dest_type)
Get DBaaS integration settings

Get DBaaS integration settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_type** | **String** |  | [required] |
**source_type** | **String** |  | [required] |
**dest_type** | **String** |  | [required] |

### Return type

[**crate::models::ListDbaasIntegrationSettings200Response**](list_dbaas_integration_settings_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_integration_types

> crate::models::ListDbaasIntegrationTypes200Response list_dbaas_integration_types()
Get DBaaS integration types

Get DBaaS integration types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDbaasIntegrationTypes200Response**](list_dbaas_integration_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_service_types

> crate::models::ListDbaasServiceTypes200Response list_dbaas_service_types()
DBaaS Service Types

List available service types for DBaaS

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDbaasServiceTypes200Response**](list_dbaas_service_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dbaas_services

> crate::models::ListDbaasServices200Response list_dbaas_services()
List DBaaS services

List DBaaS services

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListDbaasServices200Response**](list_dbaas_services_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_kafka_user_password

> crate::models::Operation reset_dbaas_kafka_user_password(service_name, username, reset_dbaas_opensearch_user_password_request)
Reset the credentials of a DBaaS Kafka user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_opensearch_user_password_request** | [**ResetDbaasOpensearchUserPasswordRequest**](ResetDbaasOpensearchUserPasswordRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_mysql_user_password

> crate::models::Operation reset_dbaas_mysql_user_password(service_name, username, reset_dbaas_mysql_user_password_request)
Reset the credentials of a DBaaS mysql user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_mysql_user_password_request** | [**ResetDbaasMysqlUserPasswordRequest**](ResetDbaasMysqlUserPasswordRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_opensearch_user_password

> crate::models::Operation reset_dbaas_opensearch_user_password(service_name, username, reset_dbaas_opensearch_user_password_request)
Reset the credentials of a DBaaS OpenSearch user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_opensearch_user_password_request** | [**ResetDbaasOpensearchUserPasswordRequest**](ResetDbaasOpensearchUserPasswordRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_dbaas_postgres_user_password

> crate::models::Operation reset_dbaas_postgres_user_password(service_name, username, reset_dbaas_opensearch_user_password_request)
Reset the credentials of a DBaaS Postgres user

If no password is provided one will be generated automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**reset_dbaas_opensearch_user_password_request** | [**ResetDbaasOpensearchUserPasswordRequest**](ResetDbaasOpensearchUserPasswordRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_grafana_maintenance

> crate::models::Operation start_dbaas_grafana_maintenance(name)
Initiate Grafana maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_kafka_maintenance

> crate::models::Operation start_dbaas_kafka_maintenance(name)
Initiate Kafka maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_mysql_maintenance

> crate::models::Operation start_dbaas_mysql_maintenance(name)
Initiate MySQL maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_opensearch_maintenance

> crate::models::Operation start_dbaas_opensearch_maintenance(name)
Initiate OpenSearch maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_pg_maintenance

> crate::models::Operation start_dbaas_pg_maintenance(name)
Initiate PostgreSQL maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_dbaas_redis_maintenance

> crate::models::Operation start_dbaas_redis_maintenance(name)
Initiate Redis maintenance update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_mysql_migration

> crate::models::Operation stop_dbaas_mysql_migration(name)
Stop a DBaaS MySQL migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_pg_migration

> crate::models::Operation stop_dbaas_pg_migration(name)
Stop a DBaaS PostgreSQL migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_dbaas_redis_migration

> crate::models::Operation stop_dbaas_redis_migration(name)
Stop a DBaaS Redis migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_integration

> crate::models::Operation update_dbaas_integration(id, update_dbaas_integration_request)
Update a existing DBaaS integration

Update a existing DBaaS integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_dbaas_integration_request** | [**UpdateDbaasIntegrationRequest**](UpdateDbaasIntegrationRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_opensearch_acl_config

> crate::models::Operation update_dbaas_opensearch_acl_config(name, dbaas_opensearch_acl_config)
Create a DBaaS OpenSearch ACL configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**dbaas_opensearch_acl_config** | [**DbaasOpensearchAclConfig**](DbaasOpensearchAclConfig.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_pg_connection_pool

> crate::models::Operation update_dbaas_pg_connection_pool(service_name, connection_pool_name, update_dbaas_pg_connection_pool_request)
Update a DBaaS PostgreSQL connection pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**connection_pool_name** | **String** |  | [required] |
**update_dbaas_pg_connection_pool_request** | [**UpdateDbaasPgConnectionPoolRequest**](UpdateDbaasPgConnectionPoolRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_postgres_allow_replication

> crate::models::DbaasPostgresUsers update_dbaas_postgres_allow_replication(service_name, username, update_dbaas_postgres_allow_replication_request)
Update access control for one service user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**username** | **String** |  | [required] |
**update_dbaas_postgres_allow_replication_request** | [**UpdateDbaasPostgresAllowReplicationRequest**](UpdateDbaasPostgresAllowReplicationRequest.md) |  | [required] |

### Return type

[**crate::models::DbaasPostgresUsers**](dbaas-postgres-users.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_grafana

> crate::models::Operation update_dbaas_service_grafana(name, update_dbaas_service_grafana_request)
Update a DBaaS Grafana service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_grafana_request** | [**UpdateDbaasServiceGrafanaRequest**](UpdateDbaasServiceGrafanaRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_kafka

> crate::models::Operation update_dbaas_service_kafka(name, update_dbaas_service_kafka_request)
Update a DBaaS Kafka service

Update a DBaaS Kafka service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_kafka_request** | [**UpdateDbaasServiceKafkaRequest**](UpdateDbaasServiceKafkaRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_mysql

> crate::models::Operation update_dbaas_service_mysql(name, update_dbaas_service_mysql_request)
Update a DBaaS MySQL service

Update a DBaaS MySQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_mysql_request** | [**UpdateDbaasServiceMysqlRequest**](UpdateDbaasServiceMysqlRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_opensearch

> crate::models::Operation update_dbaas_service_opensearch(name, update_dbaas_service_opensearch_request)
Update a DBaaS OpenSearch service

Update a DBaaS OpenSearch service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_opensearch_request** | [**UpdateDbaasServiceOpensearchRequest**](UpdateDbaasServiceOpensearchRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_pg

> crate::models::Operation update_dbaas_service_pg(name, update_dbaas_service_pg_request)
Update a DBaaS PostgreSQL service

Update a DBaaS PostgreSQL service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_pg_request** | [**UpdateDbaasServicePgRequest**](UpdateDbaasServicePgRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dbaas_service_redis

> crate::models::Operation update_dbaas_service_redis(name, update_dbaas_service_redis_request)
Update a DBaaS Redis service

Update a DBaaS Redis service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**update_dbaas_service_redis_request** | [**UpdateDbaasServiceRedisRequest**](UpdateDbaasServiceRedisRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

