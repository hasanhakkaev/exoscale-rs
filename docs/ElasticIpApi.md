# \ElasticIpApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_instance_to_elastic_ip**](ElasticIpApi.md#attach_instance_to_elastic_ip) | **PUT** /elastic-ip/{id}:attach | Attach a Compute instance to an Elastic IP
[**create_elastic_ip**](ElasticIpApi.md#create_elastic_ip) | **POST** /elastic-ip | Create an Elastic IP
[**delete_elastic_ip**](ElasticIpApi.md#delete_elastic_ip) | **DELETE** /elastic-ip/{id} | Delete an Elastic IP
[**detach_instance_from_elastic_ip**](ElasticIpApi.md#detach_instance_from_elastic_ip) | **PUT** /elastic-ip/{id}:detach | Detach a Compute instance from an Elastic IP
[**get_elastic_ip**](ElasticIpApi.md#get_elastic_ip) | **GET** /elastic-ip/{id} | Retrieve Elastic IP details
[**list_elastic_ips**](ElasticIpApi.md#list_elastic_ips) | **GET** /elastic-ip | List Elastic IPs
[**reset_elastic_ip_field**](ElasticIpApi.md#reset_elastic_ip_field) | **DELETE** /elastic-ip/{id}/{field} | Reset an Elastic IP field to its default value
[**update_elastic_ip**](ElasticIpApi.md#update_elastic_ip) | **PUT** /elastic-ip/{id} | Update an Elastic IP



## attach_instance_to_elastic_ip

> crate::models::Operation attach_instance_to_elastic_ip(id, detach_instance_from_elastic_ip_request)
Attach a Compute instance to an Elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**detach_instance_from_elastic_ip_request** | [**DetachInstanceFromElasticIpRequest**](DetachInstanceFromElasticIpRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_elastic_ip

> crate::models::Operation create_elastic_ip(create_elastic_ip_request)
Create an Elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_elastic_ip_request** | [**CreateElasticIpRequest**](CreateElasticIpRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_elastic_ip

> crate::models::Operation delete_elastic_ip(id)
Delete an Elastic IP



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


## detach_instance_from_elastic_ip

> crate::models::Operation detach_instance_from_elastic_ip(id, detach_instance_from_elastic_ip_request)
Detach a Compute instance from an Elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**detach_instance_from_elastic_ip_request** | [**DetachInstanceFromElasticIpRequest**](DetachInstanceFromElasticIpRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_elastic_ip

> crate::models::ElasticIp get_elastic_ip(id)
Retrieve Elastic IP details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::ElasticIp**](elastic-ip.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_elastic_ips

> crate::models::ListElasticIps200Response list_elastic_ips()
List Elastic IPs



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListElasticIps200Response**](list_elastic_ips_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_elastic_ip_field

> crate::models::Operation reset_elastic_ip_field(id, field)
Reset an Elastic IP field to its default value



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


## update_elastic_ip

> crate::models::Operation update_elastic_ip(id, update_elastic_ip_request)
Update an Elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_elastic_ip_request** | [**UpdateElasticIpRequest**](UpdateElasticIpRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

