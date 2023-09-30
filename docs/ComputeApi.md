# \ComputeApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reset_load_balancer_service_field**](ComputeApi.md#reset_load_balancer_service_field) | **DELETE** /load-balancer/{id}/service/{service_id}/{field} | Reset a Load Balancer Service field to its default value



## reset_load_balancer_service_field

> crate::models::Operation reset_load_balancer_service_field(id, service_id, field)
Reset a Load Balancer Service field to its default value



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

