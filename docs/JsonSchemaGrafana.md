# JsonSchemaGrafana

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_embedding** | Option<**bool**> |  | [optional]
**cookie_samesite** | Option<**String**> |  | [optional]
**dashboard_previews_enabled** | Option<**bool**> | This feature is new in Grafana 9 and is quite resource intensive. It may cause low-end plans to work more slowly while the dashboard previews are rendering. | [optional]
**metrics_enabled** | Option<**bool**> |  | [optional]
**auth_azuread** | Option<[**models::AzureAdOAuthIntegration**](Azure_AD_OAuth_integration.md)> |  | [optional]
**alerting_enabled** | Option<**bool**> |  | [optional]
**wal** | Option<**bool**> | Setting to enable/disable Write-Ahead Logging. The default value is false (disabled). | [optional]
**unified_alerting_enabled** | Option<**bool**> |  | [optional]
**auth_github** | Option<[**models::GithubAuthIntegration**](Github_Auth_integration.md)> |  | [optional]
**user_auto_assign_org** | Option<**bool**> |  | [optional]
**dataproxy_send_user_header** | Option<**bool**> |  | [optional]
**google_analytics_ua_id** | Option<**String**> |  | [optional]
**dashboards_versions_to_keep** | Option<**i32**> |  | [optional]
**editors_can_admin** | Option<**bool**> |  | [optional]
**smtp_server** | Option<[**models::SmtpServerSettings**](SMTP_server_settings.md)> |  | [optional]
**auth_gitlab** | Option<[**models::GitLabAuthIntegration**](GitLab_Auth_integration.md)> |  | [optional]
**alerting_nodata_or_nullvalues** | Option<**String**> |  | [optional]
**auth_basic_enabled** | Option<**bool**> |  | [optional]
**date_formats** | Option<[**models::GrafanaDateFormatSpecifications**](Grafana_date_format_specifications.md)> |  | [optional]
**service_log** | Option<**bool**> | Store logs for the service so that they are available in the HTTP API and console. | [optional]
**disable_gravatar** | Option<**bool**> |  | [optional]
**user_auto_assign_org_role** | Option<**String**> |  | [optional]
**dataproxy_timeout** | Option<**i32**> |  | [optional]
**viewers_can_edit** | Option<**bool**> |  | [optional]
**dashboards_min_refresh_interval** | Option<**String**> | Signed sequence of decimal numbers, followed by a unit suffix (ms, s, m, h, d), e.g. 30s, 1h | [optional]
**auth_google** | Option<[**models::GoogleAuthIntegration**](Google_Auth_integration.md)> |  | [optional]
**oauth_allow_insecure_email_lookup** | Option<**bool**> |  | [optional]
**alerting_max_annotations_to_keep** | Option<**i32**> |  | [optional]
**auth_generic_oauth** | Option<[**models::GenericOAuthIntegration**](Generic_OAuth_integration.md)> |  | [optional]
**custom_domain** | Option<**String**> | Serve the web frontend using a custom CNAME pointing to the Aiven DNS name | [optional]
**alerting_error_or_timeout** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


