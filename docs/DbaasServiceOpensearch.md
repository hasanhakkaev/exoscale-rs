# DbaasServiceOpensearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | DbaaS service description | [optional]
**max_index_count** | Option<**i64**> | Maximum number of indexes to keep before deleting the oldest one | [optional]
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**i64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**models::DbaasServiceOpensearchConnectionInfo**](dbaas_service_opensearch_connection_info.md)> |  | [optional]
**node_cpu_count** | Option<**i64**> | Number of CPUs for each node | [optional]
**integrations** | Option<[**Vec<models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**node_states** | Option<[**Vec<models::DbaasNodeState>**](dbaas-node-state.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**keep_index_refresh_interval** | Option<**bool**> | Aiven automation resets index.refresh_interval to default value for every index to be sure that indices are always visible to search. If it doesn't fit your case, you can disable this by setting up this flag to true. | [optional]
**r#type** | **String** |  | 
**state** | Option<[**models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**ip_filter** | Option<**Vec<String>**> | Allowed CIDR address blocks for incoming connections | [optional]
**backups** | Option<[**Vec<models::DbaasServiceBackup>**](dbaas-service-backup.md)> | List of backups for the service | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**components** | Option<[**Vec<models::DbaasServiceMysqlComponentsInner>**](dbaas_service_mysql_components_inner.md)> | Service component information objects | [optional]
**index_patterns** | Option<[**Vec<models::UpdateDbaasServiceOpensearchRequestIndexPatternsInner>**](update_dbaas_service_opensearch_request_index_patterns_inner.md)> | Allows you to create glob style patterns and set a max number of indexes matching this pattern you want to keep. Creating indexes exceeding this value will cause the oldest one to get deleted. You could for example create a pattern looking like 'logs.?' and then create index logs.1, logs.2 etc, it will delete logs.1 once you create logs.6. Do note 'logs.?' does not apply to logs.10. Note: Setting max_index_count to 0 will do nothing and the pattern gets ignored. | [optional]
**maintenance** | Option<[**models::DbaasServiceMaintenance**](dbaas-service-maintenance.md)> |  | [optional]
**index_template** | Option<[**models::UpdateDbaasServiceOpensearchRequestIndexTemplate**](update_dbaas_service_opensearch_request_index_template.md)> |  | [optional]
**disk_size** | Option<**i64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**i64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**opensearch_settings** | Option<[**models::JsonSchemaOpensearch**](json-schema-opensearch.md)> |  | [optional]
**uri_params** | Option<[**serde_json::Value**](.md)> | service_uri parameterized into key-value pairs | [optional]
**version** | Option<**String**> | OpenSearch version | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**opensearch_dashboards** | Option<[**models::UpdateDbaasServiceOpensearchRequestOpensearchDashboards**](update_dbaas_service_opensearch_request_opensearch_dashboards.md)> |  | [optional]
**users** | Option<[**Vec<models::DbaasServiceGrafanaUsersInner>**](dbaas_service_grafana_users_inner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


