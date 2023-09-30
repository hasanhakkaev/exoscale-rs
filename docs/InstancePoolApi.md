# \InstancePoolApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_instance_pool**](InstancePoolApi.md#create_instance_pool) | **POST** /instance-pool | Create an Instance Pool
[**delete_instance_pool**](InstancePoolApi.md#delete_instance_pool) | **DELETE** /instance-pool/{id} | Delete an Instance Pool
[**evict_instance_pool_members**](InstancePoolApi.md#evict_instance_pool_members) | **PUT** /instance-pool/{id}:evict | Evict Instance Pool members
[**get_instance_pool**](InstancePoolApi.md#get_instance_pool) | **GET** /instance-pool/{id} | Retrieve Instance Pool details
[**list_instance_pools**](InstancePoolApi.md#list_instance_pools) | **GET** /instance-pool | List Instance Pools
[**reset_instance_pool_field**](InstancePoolApi.md#reset_instance_pool_field) | **DELETE** /instance-pool/{id}/{field} | Reset an Instance Pool field to its default value
[**scale_instance_pool**](InstancePoolApi.md#scale_instance_pool) | **PUT** /instance-pool/{id}:scale | Scale an Instance Pool
[**update_instance_pool**](InstancePoolApi.md#update_instance_pool) | **PUT** /instance-pool/{id} | Update an Instance Pool



## create_instance_pool

> crate::models::Operation create_instance_pool(create_instance_pool_request)
Create an Instance Pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_instance_pool_request** | [**CreateInstancePoolRequest**](CreateInstancePoolRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance_pool

> crate::models::Operation delete_instance_pool(id)
Delete an Instance Pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evict_instance_pool_members

> crate::models::Operation evict_instance_pool_members(id, evict_instance_pool_members_request)
Evict Instance Pool members

This operation evicts the specified Compute instances member from the Instance Pool, shrinking it to `&lt;current pool size&gt; - &lt;# evicted members&gt;`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**evict_instance_pool_members_request** | [**EvictInstancePoolMembersRequest**](EvictInstancePoolMembersRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_pool

> crate::models::InstancePool get_instance_pool(id)
Retrieve Instance Pool details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::InstancePool**](instance-pool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instance_pools

> crate::models::ListInstancePools200Response list_instance_pools()
List Instance Pools



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListInstancePools200Response**](list_instance_pools_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_instance_pool_field

> crate::models::Operation reset_instance_pool_field(id, field)
Reset an Instance Pool field to its default value



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_instance_pool

> crate::models::Operation scale_instance_pool(id, scale_instance_pool_request)
Scale an Instance Pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**scale_instance_pool_request** | [**ScaleInstancePoolRequest**](ScaleInstancePoolRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_pool

> crate::models::Operation update_instance_pool(id, update_instance_pool_request)
Update an Instance Pool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_instance_pool_request** | [**UpdateInstancePoolRequest**](UpdateInstancePoolRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

