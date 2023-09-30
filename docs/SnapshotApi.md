# \SnapshotApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot**](SnapshotApi.md#create_snapshot) | **POST** /instance/{id}:create-snapshot | Create a Snapshot of a Compute instance
[**delete_snapshot**](SnapshotApi.md#delete_snapshot) | **DELETE** /snapshot/{id} | Delete a Snapshot
[**export_snapshot**](SnapshotApi.md#export_snapshot) | **POST** /snapshot/{id}:export | Export a Snapshot
[**get_snapshot**](SnapshotApi.md#get_snapshot) | **GET** /snapshot/{id} | Retrieve Snapshot details
[**list_snapshots**](SnapshotApi.md#list_snapshots) | **GET** /snapshot | List Snapshots
[**promote_snapshot_to_template**](SnapshotApi.md#promote_snapshot_to_template) | **POST** /snapshot/{id}:promote | Promote a Snapshot to a Template



## create_snapshot

> crate::models::Operation create_snapshot(id)
Create a Snapshot of a Compute instance



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


## delete_snapshot

> crate::models::Operation delete_snapshot(id)
Delete a Snapshot



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


## export_snapshot

> crate::models::Operation export_snapshot(id)
Export a Snapshot



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


## get_snapshot

> crate::models::Snapshot get_snapshot(id)
Retrieve Snapshot details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Snapshot**](snapshot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots

> crate::models::ListSnapshots200Response list_snapshots()
List Snapshots



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListSnapshots200Response**](list_snapshots_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_snapshot_to_template

> crate::models::Operation promote_snapshot_to_template(id, promote_snapshot_to_template_request)
Promote a Snapshot to a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**promote_snapshot_to_template_request** | [**PromoteSnapshotToTemplateRequest**](PromoteSnapshotToTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

