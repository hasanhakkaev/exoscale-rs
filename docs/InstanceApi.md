# \InstanceApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_instance_protection**](InstanceApi.md#add_instance_protection) | **PUT** /instance/{id}:add-protection | Set instance destruction protection
[**create_instance**](InstanceApi.md#create_instance) | **POST** /instance | Create a Compute instance
[**create_snapshot**](InstanceApi.md#create_snapshot) | **POST** /instance/{id}:create-snapshot | Create a Snapshot of a Compute instance
[**delete_instance**](InstanceApi.md#delete_instance) | **DELETE** /instance/{id} | Delete a Compute instance
[**get_instance**](InstanceApi.md#get_instance) | **GET** /instance/{id} | Retrieve Compute instance details
[**list_instances**](InstanceApi.md#list_instances) | **GET** /instance | List Compute instances
[**reboot_instance**](InstanceApi.md#reboot_instance) | **PUT** /instance/{id}:reboot | Reboot a Compute instance
[**remove_instance_protection**](InstanceApi.md#remove_instance_protection) | **PUT** /instance/{id}:remove-protection | Remove instance destruction protection
[**reset_instance**](InstanceApi.md#reset_instance) | **PUT** /instance/{id}:reset | Reset a Compute instance to a base/target template
[**reset_instance_field**](InstanceApi.md#reset_instance_field) | **DELETE** /instance/{id}/{field} | Reset Instance field
[**reset_instance_password**](InstanceApi.md#reset_instance_password) | **PUT** /instance/{id}:reset-password | Reset a compute instance password
[**resize_instance_disk**](InstanceApi.md#resize_instance_disk) | **PUT** /instance/{id}:resize-disk | Resize a Compute instance disk
[**reveal_instance_password**](InstanceApi.md#reveal_instance_password) | **GET** /instance/{id}:password | Reveal the password used during instance creation or the latest password reset.
[**revert_instance_to_snapshot**](InstanceApi.md#revert_instance_to_snapshot) | **POST** /instance/{instance_id}:revert-snapshot | Revert a snapshot for an instance
[**scale_instance**](InstanceApi.md#scale_instance) | **PUT** /instance/{id}:scale | Scale a Compute instance to a new Instance Type
[**start_instance**](InstanceApi.md#start_instance) | **PUT** /instance/{id}:start | Start a Compute instance
[**stop_instance**](InstanceApi.md#stop_instance) | **PUT** /instance/{id}:stop | Stop a Compute instance
[**update_instance**](InstanceApi.md#update_instance) | **PUT** /instance/{id} | Update a Compute instance



## add_instance_protection

> serde_json::Value add_instance_protection(id)
Set instance destruction protection



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> crate::models::Operation create_instance(create_instance_request)
Create a Compute instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_instance_request** | [**CreateInstanceRequest**](CreateInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## delete_instance

> crate::models::Operation delete_instance(id)
Delete a Compute instance



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


## get_instance

> crate::models::Instance get_instance(id)
Retrieve Compute instance details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Instance**](instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances

> crate::models::ListInstances200Response list_instances(manager_id, manager_type, ip_address)
List Compute instances



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manager_id** | Option<**uuid::Uuid**> |  |  |
**manager_type** | Option<**String**> |  |  |
**ip_address** | Option<**String**> |  |  |

### Return type

[**crate::models::ListInstances200Response**](list_instances_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_instance

> crate::models::Operation reboot_instance(id)
Reboot a Compute instance



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


## remove_instance_protection

> serde_json::Value remove_instance_protection(id)
Remove instance destruction protection



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_instance

> crate::models::Operation reset_instance(id, reset_instance_request)
Reset a Compute instance to a base/target template

This operation re-installs a Compute instance to a base template. If target template is provided it will be used to recreated instance from. Warning: the operation wipes all data stored on the disk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**reset_instance_request** | [**ResetInstanceRequest**](ResetInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_instance_field

> crate::models::Operation reset_instance_field(id, field)
Reset Instance field



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


## reset_instance_password

> crate::models::Operation reset_instance_password(id)
Reset a compute instance password



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


## resize_instance_disk

> crate::models::Operation resize_instance_disk(id, resize_instance_disk_request)
Resize a Compute instance disk

This operation resizes a Compute instance's disk volume. Note: the disk can only grow, cannot be shrunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**resize_instance_disk_request** | [**ResizeInstanceDiskRequest**](ResizeInstanceDiskRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_instance_password

> crate::models::InstancePassword reveal_instance_password(id)
Reveal the password used during instance creation or the latest password reset.

Reveal the password used during instance creation or the latest password reset.             This is only available for VMs created against templates having the `password-enabled`             property set to `true`.              Passwords are transiently stored for at most 24 hours and intended to be retrieved shortly after             creation or resets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::InstancePassword**](instance-password.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revert_instance_to_snapshot

> crate::models::Operation revert_instance_to_snapshot(instance_id, revert_instance_to_snapshot_request)
Revert a snapshot for an instance

This operation reverts the snapshot to the Compute instance volume, restoring stored data as it was at the time of the snapshot. The Compute instance must be previously stopped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **uuid::Uuid** |  | [required] |
**revert_instance_to_snapshot_request** | [**RevertInstanceToSnapshotRequest**](RevertInstanceToSnapshotRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_instance

> crate::models::Operation scale_instance(id, scale_instance_request)
Scale a Compute instance to a new Instance Type

This operation changes the Compute instance's type. Note: the new Instance Type must be within the same family (e.g. a standard instance cannot be scaled to gpu2 or storage).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**scale_instance_request** | [**ScaleInstanceRequest**](ScaleInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_instance

> crate::models::Operation start_instance(id, start_instance_request)
Start a Compute instance

This operation starts a virtual machine, potentially using a rescue profile if specified

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**start_instance_request** | [**StartInstanceRequest**](StartInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_instance

> crate::models::Operation stop_instance(id)
Stop a Compute instance



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


## update_instance

> crate::models::Operation update_instance(id, update_instance_request)
Update a Compute instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_instance_request** | [**UpdateInstanceRequest**](UpdateInstanceRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

