# CreateDbaasServiceOpensearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_index_count** | Option<**i64**> | Maximum number of indexes to keep before deleting the oldest one | [optional]
**keep_index_refresh_interval** | Option<**bool**> | Aiven automation resets index.refresh_interval to default value for every index to be sure that indices are always visible to search. If it doesn't fit your case, you can disable this by setting up this flag to true. | [optional]
**ip_filter** | Option<**Vec<String>**> | Allow incoming connections from CIDR address block, e.g. '10.20.0.0/16' | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**fork_from_service** | Option<**String**> |  | [optional]
**index_patterns** | Option<[**Vec<models::UpdateDbaasServiceOpensearchRequestIndexPatternsInner>**](update_dbaas_service_opensearch_request_index_patterns_inner.md)> | Allows you to create glob style patterns and set a max number of indexes matching this pattern you want to keep. Creating indexes exceeding this value will cause the oldest one to get deleted. You could for example create a pattern looking like 'logs.?' and then create index logs.1, logs.2 etc, it will delete logs.1 once you create logs.6. Do note 'logs.?' does not apply to logs.10. Note: Setting max_index_count to 0 will do nothing and the pattern gets ignored. | [optional]
**maintenance** | Option<[**models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**index_template** | Option<[**models::UpdateDbaasServiceOpensearchRequestIndexTemplate**](update_dbaas_service_opensearch_request_index_template.md)> |  | [optional]
**opensearch_settings** | Option<[**models::JsonSchemaOpensearch**](json-schema-opensearch.md)> |  | [optional]
**version** | Option<**String**> | OpenSearch major version | [optional]
**recovery_backup_name** | Option<**String**> | Name of a backup to recover from for services that support backup names | [optional]
**plan** | **String** | Subscription plan | 
**opensearch_dashboards** | Option<[**models::UpdateDbaasServiceOpensearchRequestOpensearchDashboards**](update_dbaas_service_opensearch_request_opensearch_dashboards.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


