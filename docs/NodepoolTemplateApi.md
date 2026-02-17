# \NodepoolTemplateApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_active_nodepool_template**](NodepoolTemplateApi.md#get_active_nodepool_template) | **GET** /sks-template/{kube_version}/{variant} | 



## get_active_nodepool_template

> models::GetActiveNodepoolTemplate200Response get_active_nodepool_template(kube_version, variant)


Get the active template for a given kube version and variant (standard | nvidia)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kube_version** | **String** |  | [required] |
**variant** | **String** |  | [required] |

### Return type

[**models::GetActiveNodepoolTemplate200Response**](get_active_nodepool_template_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

