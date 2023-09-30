# UpdateDbaasServiceGrafanaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maintenance** | Option<[**crate::models::UpdateDbaasServiceMysqlRequestMaintenance**](update_dbaas_service_mysql_request_maintenance.md)> |  | [optional]
**plan** | Option<**String**> | Subscription plan | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**grafana_settings** | Option<[**serde_json::Value**](.md)> | Grafana specific settings | [optional]
**ip_filter** | Option<**Vec<String>**> | Allowed CIDR address blocks for incoming connections | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


