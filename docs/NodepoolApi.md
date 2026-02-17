# \NodepoolApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sks_nodepool**](NodepoolApi.md#create_sks_nodepool) | **POST** /sks-cluster/{id}/nodepool | Create a new SKS Nodepool
[**delete_sks_nodepool**](NodepoolApi.md#delete_sks_nodepool) | **DELETE** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Delete an SKS Nodepool
[**evict_sks_nodepool_members**](NodepoolApi.md#evict_sks_nodepool_members) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id}:evict | Evict Nodepool members
[**get_sks_nodepool**](NodepoolApi.md#get_sks_nodepool) | **GET** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Retrieve SKS Nodepool details
[**scale_sks_nodepool**](NodepoolApi.md#scale_sks_nodepool) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id}:scale | Scale a SKS Nodepool
[**update_sks_nodepool**](NodepoolApi.md#update_sks_nodepool) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Update an SKS Nodepool



## create_sks_nodepool

> models::Operation create_sks_nodepool(id, create_sks_nodepool_request)
Create a new SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**create_sks_nodepool_request** | [**CreateSksNodepoolRequest**](CreateSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sks_nodepool

> models::Operation delete_sks_nodepool(id, sks_nodepool_id)
Delete an SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evict_sks_nodepool_members

> models::Operation evict_sks_nodepool_members(id, sks_nodepool_id, evict_sks_nodepool_members_request)
Evict Nodepool members

This operation evicts the specified Compute instances member from the Nodepool, shrinking it to `&lt;current nodepool size&gt; - &lt;# evicted members&gt;`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**evict_sks_nodepool_members_request** | [**EvictSksNodepoolMembersRequest**](EvictSksNodepoolMembersRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sks_nodepool

> models::SksNodepool get_sks_nodepool(id, sks_nodepool_id)
Retrieve SKS Nodepool details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SksNodepool**](sks-nodepool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_sks_nodepool

> models::Operation scale_sks_nodepool(id, sks_nodepool_id, scale_sks_nodepool_request)
Scale a SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**scale_sks_nodepool_request** | [**ScaleSksNodepoolRequest**](ScaleSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sks_nodepool

> models::Operation update_sks_nodepool(id, sks_nodepool_id, update_sks_nodepool_request)
Update an SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**update_sks_nodepool_request** | [**UpdateSksNodepoolRequest**](UpdateSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

