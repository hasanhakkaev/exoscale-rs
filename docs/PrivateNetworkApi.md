# \PrivateNetworkApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_instance_to_private_network**](PrivateNetworkApi.md#attach_instance_to_private_network) | **PUT** /private-network/{id}:attach | Attach a Compute instance to a Private Network
[**create_private_network**](PrivateNetworkApi.md#create_private_network) | **POST** /private-network | Create a Private Network
[**delete_private_network**](PrivateNetworkApi.md#delete_private_network) | **DELETE** /private-network/{id} | Delete a Private Network
[**detach_instance_from_private_network**](PrivateNetworkApi.md#detach_instance_from_private_network) | **PUT** /private-network/{id}:detach | Detach a Compute instance from a Private Network
[**get_private_network**](PrivateNetworkApi.md#get_private_network) | **GET** /private-network/{id} | Retrieve Private Network details
[**list_private_networks**](PrivateNetworkApi.md#list_private_networks) | **GET** /private-network | List Private Networks
[**reset_private_network_field**](PrivateNetworkApi.md#reset_private_network_field) | **DELETE** /private-network/{id}/{field} | Reset Private Network field
[**update_private_network**](PrivateNetworkApi.md#update_private_network) | **PUT** /private-network/{id} | Update a Private Network
[**update_private_network_instance_ip**](PrivateNetworkApi.md#update_private_network_instance_ip) | **PUT** /private-network/{id}:update-ip | Update the IP address of an instance attached to a managed private network



## attach_instance_to_private_network

> models::Operation attach_instance_to_private_network(id, attach_instance_to_private_network_request)
Attach a Compute instance to a Private Network



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**attach_instance_to_private_network_request** | [**AttachInstanceToPrivateNetworkRequest**](AttachInstanceToPrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_private_network

> models::Operation create_private_network(create_private_network_request)
Create a Private Network



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_private_network_request** | [**CreatePrivateNetworkRequest**](CreatePrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_private_network

> models::Operation delete_private_network(id)
Delete a Private Network



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


## detach_instance_from_private_network

> models::Operation detach_instance_from_private_network(id, detach_instance_from_private_network_request)
Detach a Compute instance from a Private Network



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**detach_instance_from_private_network_request** | [**DetachInstanceFromPrivateNetworkRequest**](DetachInstanceFromPrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_private_network

> models::PrivateNetwork get_private_network(id)
Retrieve Private Network details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PrivateNetwork**](private-network.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_private_networks

> models::ListPrivateNetworks200Response list_private_networks()
List Private Networks



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListPrivateNetworks200Response**](list_private_networks_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_private_network_field

> models::Operation reset_private_network_field(id, field)
Reset Private Network field



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_network

> models::Operation update_private_network(id, update_private_network_request)
Update a Private Network



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_private_network_request** | [**UpdatePrivateNetworkRequest**](UpdatePrivateNetworkRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_private_network_instance_ip

> models::Operation update_private_network_instance_ip(id, update_private_network_instance_ip_request)
Update the IP address of an instance attached to a managed private network



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_private_network_instance_ip_request** | [**UpdatePrivateNetworkInstanceIpRequest**](UpdatePrivateNetworkInstanceIpRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

