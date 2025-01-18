# \AntiAffinityGroupApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_anti_affinity_group**](AntiAffinityGroupApi.md#create_anti_affinity_group) | **POST** /anti-affinity-group | Create an Anti-affinity Group
[**delete_anti_affinity_group**](AntiAffinityGroupApi.md#delete_anti_affinity_group) | **DELETE** /anti-affinity-group/{id} | Delete an Anti-affinity Group
[**get_anti_affinity_group**](AntiAffinityGroupApi.md#get_anti_affinity_group) | **GET** /anti-affinity-group/{id} | Retrieve Anti-affinity Group details
[**list_anti_affinity_groups**](AntiAffinityGroupApi.md#list_anti_affinity_groups) | **GET** /anti-affinity-group | List Anti-affinity Groups



## create_anti_affinity_group

> models::Operation create_anti_affinity_group(create_anti_affinity_group_request)
Create an Anti-affinity Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_anti_affinity_group_request** | [**CreateAntiAffinityGroupRequest**](CreateAntiAffinityGroupRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_anti_affinity_group

> models::Operation delete_anti_affinity_group(id)
Delete an Anti-affinity Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_anti_affinity_group

> models::AntiAffinityGroup get_anti_affinity_group(id)
Retrieve Anti-affinity Group details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AntiAffinityGroup**](anti-affinity-group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_anti_affinity_groups

> models::ListAntiAffinityGroups200Response list_anti_affinity_groups()
List Anti-affinity Groups



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAntiAffinityGroups200Response**](list_anti_affinity_groups_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

