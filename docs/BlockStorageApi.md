# \BlockStorageApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_block_storage_volume_to_instance**](BlockStorageApi.md#attach_block_storage_volume_to_instance) | **PUT** /block-storage/{id}:attach | Attach block storage volume to an instance
[**create_block_storage_snapshot**](BlockStorageApi.md#create_block_storage_snapshot) | **POST** /block-storage/{id}:create-snapshot | Create a block storage snapshot
[**create_block_storage_volume**](BlockStorageApi.md#create_block_storage_volume) | **POST** /block-storage | Create a block storage volume
[**delete_block_storage_snapshot**](BlockStorageApi.md#delete_block_storage_snapshot) | **DELETE** /block-storage-snapshot/{id} | Delete a block storage snapshot, data will be unrecoverable
[**delete_block_storage_volume**](BlockStorageApi.md#delete_block_storage_volume) | **DELETE** /block-storage/{id} | Delete a block storage volume, data will be unrecoverable
[**detach_block_storage_volume**](BlockStorageApi.md#detach_block_storage_volume) | **PUT** /block-storage/{id}:detach | Detach block storage volume
[**get_block_storage_snapshot**](BlockStorageApi.md#get_block_storage_snapshot) | **GET** /block-storage-snapshot/{id} | Retrieve block storage snapshot details
[**get_block_storage_volume**](BlockStorageApi.md#get_block_storage_volume) | **GET** /block-storage/{id} | Retrieve block storage volume details
[**list_block_storage_snapshots**](BlockStorageApi.md#list_block_storage_snapshots) | **GET** /block-storage-snapshot | List block storage snapshots
[**list_block_storage_volumes**](BlockStorageApi.md#list_block_storage_volumes) | **GET** /block-storage | List block storage volumes
[**resize_block_storage_volume**](BlockStorageApi.md#resize_block_storage_volume) | **PUT** /block-storage/{id}:resize-volume | Resize a block storage volume
[**update_block_storage_snapshot**](BlockStorageApi.md#update_block_storage_snapshot) | **PUT** /block-storage-snapshot/{id} | Update block storage volume snapshot
[**update_block_storage_volume**](BlockStorageApi.md#update_block_storage_volume) | **PUT** /block-storage/{id} | Update block storage volume



## attach_block_storage_volume_to_instance

> models::Operation attach_block_storage_volume_to_instance(id, attach_block_storage_volume_to_instance_request)
Attach block storage volume to an instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**attach_block_storage_volume_to_instance_request** | [**AttachBlockStorageVolumeToInstanceRequest**](AttachBlockStorageVolumeToInstanceRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_block_storage_snapshot

> models::Operation create_block_storage_snapshot(id, create_block_storage_snapshot_request)
Create a block storage snapshot



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**create_block_storage_snapshot_request** | [**CreateBlockStorageSnapshotRequest**](CreateBlockStorageSnapshotRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_block_storage_volume

> models::Operation create_block_storage_volume(create_block_storage_volume_request)
Create a block storage volume



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_block_storage_volume_request** | [**CreateBlockStorageVolumeRequest**](CreateBlockStorageVolumeRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_block_storage_snapshot

> models::Operation delete_block_storage_snapshot(id)
Delete a block storage snapshot, data will be unrecoverable



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


## delete_block_storage_volume

> models::Operation delete_block_storage_volume(id)
Delete a block storage volume, data will be unrecoverable



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


## detach_block_storage_volume

> models::Operation detach_block_storage_volume(id)
Detach block storage volume



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


## get_block_storage_snapshot

> models::BlockStorageSnapshot get_block_storage_snapshot(id)
Retrieve block storage snapshot details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::BlockStorageSnapshot**](block-storage-snapshot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_storage_volume

> models::BlockStorageVolume get_block_storage_volume(id)
Retrieve block storage volume details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::BlockStorageVolume**](block-storage-volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_block_storage_snapshots

> models::ListBlockStorageSnapshots200Response list_block_storage_snapshots()
List block storage snapshots



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListBlockStorageSnapshots200Response**](list_block_storage_snapshots_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_block_storage_volumes

> models::ListBlockStorageVolumes200Response list_block_storage_volumes(instance_id)
List block storage volumes



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**models::ListBlockStorageVolumes200Response**](list_block_storage_volumes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resize_block_storage_volume

> models::BlockStorageVolume resize_block_storage_volume(id, resize_block_storage_volume_request)
Resize a block storage volume

This operation resizes a Block storage volume. Note: the volume can only grow, cannot be shrunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**resize_block_storage_volume_request** | [**ResizeBlockStorageVolumeRequest**](ResizeBlockStorageVolumeRequest.md) |  | [required] |

### Return type

[**models::BlockStorageVolume**](block-storage-volume.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_block_storage_snapshot

> models::Operation update_block_storage_snapshot(id, update_block_storage_snapshot_request)
Update block storage volume snapshot



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_block_storage_snapshot_request** | [**UpdateBlockStorageSnapshotRequest**](UpdateBlockStorageSnapshotRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_block_storage_volume

> models::Operation update_block_storage_volume(id, update_block_storage_volume_request)
Update block storage volume



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_block_storage_volume_request** | [**UpdateBlockStorageVolumeRequest**](UpdateBlockStorageVolumeRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

