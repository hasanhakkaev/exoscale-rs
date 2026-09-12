# \VpcSubnetApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_instance_to_subnet**](VpcSubnetApi.md#attach_instance_to_subnet) | **PUT** /vpc/{vpc_id}/subnet/{subnet_id}/attach | [BETA] Attach a Compute instance to a Subnet
[**create_subnet**](VpcSubnetApi.md#create_subnet) | **POST** /vpc/{vpc_id}/subnet | [BETA] Create a Subnet
[**delete_subnet**](VpcSubnetApi.md#delete_subnet) | **DELETE** /vpc/{vpc_id}/subnet/{id} | [BETA] Delete a Subnet
[**detach_instance_from_subnet**](VpcSubnetApi.md#detach_instance_from_subnet) | **PUT** /vpc/{vpc_id}/subnet/{subnet_id}/detach | [BETA] Detach a Compute instance from a Subnet
[**get_subnet**](VpcSubnetApi.md#get_subnet) | **GET** /vpc/{vpc_id}/subnet/{id} | [BETA] Retrieve Subnet details
[**list_subnets**](VpcSubnetApi.md#list_subnets) | **GET** /vpc/{vpc_id}/subnet | [BETA] List Subnets
[**update_subnet**](VpcSubnetApi.md#update_subnet) | **PUT** /vpc/{vpc_id}/subnet/{id} | [BETA] Update a Subnet



## attach_instance_to_subnet

> models::Operation attach_instance_to_subnet(vpc_id, subnet_id, attach_instance_to_subnet_request)
[BETA] Attach a Compute instance to a Subnet



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**subnet_id** | **uuid::Uuid** |  | [required] |
**attach_instance_to_subnet_request** | [**AttachInstanceToSubnetRequest**](AttachInstanceToSubnetRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subnet

> models::Operation create_subnet(vpc_id, create_subnet_request)
[BETA] Create a Subnet



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**create_subnet_request** | [**CreateSubnetRequest**](CreateSubnetRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subnet

> serde_json::Value delete_subnet(vpc_id, id)
[BETA] Delete a Subnet



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## detach_instance_from_subnet

> models::Operation detach_instance_from_subnet(vpc_id, subnet_id, attach_block_storage_volume_to_instance_request)
[BETA] Detach a Compute instance from a Subnet



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**subnet_id** | **uuid::Uuid** |  | [required] |
**attach_block_storage_volume_to_instance_request** | [**AttachBlockStorageVolumeToInstanceRequest**](AttachBlockStorageVolumeToInstanceRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subnet

> models::Subnet get_subnet(vpc_id, id)
[BETA] Retrieve Subnet details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Subnet**](subnet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subnets

> models::ListSubnets200Response list_subnets(vpc_id)
[BETA] List Subnets



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListSubnets200Response**](list_subnets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subnet

> models::Subnet update_subnet(vpc_id, id, update_subnet_request)
[BETA] Update a Subnet



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |
**update_subnet_request** | [**UpdateSubnetRequest**](UpdateSubnetRequest.md) |  | [required] |

### Return type

[**models::Subnet**](subnet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

