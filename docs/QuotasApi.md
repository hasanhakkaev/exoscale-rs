# \QuotasApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_quota**](QuotasApi.md#get_quota) | **GET** /quota/{entity} | Retrieve Resource Quota
[**list_quotas**](QuotasApi.md#list_quotas) | **GET** /quota | List Organization Quotas



## get_quota

> crate::models::Quota get_quota(entity)
Retrieve Resource Quota



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |

### Return type

[**crate::models::Quota**](quota.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_quotas

> crate::models::ListQuotas200Response list_quotas()
List Organization Quotas



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListQuotas200Response**](list_quotas_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

