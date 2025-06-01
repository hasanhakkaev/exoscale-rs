# \OrganizationApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_env_impact**](OrganizationApi.md#get_env_impact) | **GET** /env-impact | [BETA] Retrieve organization environmental impact reports
[**get_organization**](OrganizationApi.md#get_organization) | **GET** /organization | Retrieve an organization
[**get_usage_report**](OrganizationApi.md#get_usage_report) | **GET** /usage-report | Retrieve organization usage reports



## get_env_impact

> models::GetEnvImpact200Response get_env_impact(period)
[BETA] Retrieve organization environmental impact reports

[BETA] Returns environmental impact reports for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> |  |  |

### Return type

[**models::GetEnvImpact200Response**](get_env_impact_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::Organization get_organization()
Retrieve an organization



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Organization**](organization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_report

> models::GetUsageReport200Response get_usage_report(period)
Retrieve organization usage reports

Returns aggregated usage reports for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**period** | Option<**String**> |  |  |

### Return type

[**models::GetUsageReport200Response**](get_usage_report_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

