# \InstanceTypeApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instance_type**](InstanceTypeApi.md#get_instance_type) | **GET** /instance-type/{id} | Retrieve Instance Type details
[**list_instance_types**](InstanceTypeApi.md#list_instance_types) | **GET** /instance-type | List Compute instance Types



## get_instance_type

> models::InstanceType get_instance_type(id)
Retrieve Instance Type details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::InstanceType**](instance-type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_types

> models::ListInstanceTypes200Response list_instance_types()
List Compute instance Types



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListInstanceTypes200Response**](list_instance_types_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

