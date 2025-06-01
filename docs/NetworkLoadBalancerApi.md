# \NetworkLoadBalancerApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_service_to_load_balancer**](NetworkLoadBalancerApi.md#add_service_to_load_balancer) | **POST** /load-balancer/{id}/service | Add a Load Balancer Service
[**create_load_balancer**](NetworkLoadBalancerApi.md#create_load_balancer) | **POST** /load-balancer | Create a Load Balancer
[**delete_load_balancer**](NetworkLoadBalancerApi.md#delete_load_balancer) | **DELETE** /load-balancer/{id} | Delete a Load Balancer
[**delete_load_balancer_service**](NetworkLoadBalancerApi.md#delete_load_balancer_service) | **DELETE** /load-balancer/{id}/service/{service_id} | Delete a Load Balancer Service
[**get_load_balancer**](NetworkLoadBalancerApi.md#get_load_balancer) | **GET** /load-balancer/{id} | Retrieve Load Balancer details
[**get_load_balancer_service**](NetworkLoadBalancerApi.md#get_load_balancer_service) | **GET** /load-balancer/{id}/service/{service_id} | Retrieve Load Balancer Service details
[**list_load_balancers**](NetworkLoadBalancerApi.md#list_load_balancers) | **GET** /load-balancer | List Load Balancers
[**reset_load_balancer_field**](NetworkLoadBalancerApi.md#reset_load_balancer_field) | **DELETE** /load-balancer/{id}/{field} | Reset a Load Balancer field to its default value
[**reset_load_balancer_service_field**](NetworkLoadBalancerApi.md#reset_load_balancer_service_field) | **DELETE** /load-balancer/{id}/service/{service_id}/{field} | Reset a Load Balancer Service field to its default value
[**update_load_balancer**](NetworkLoadBalancerApi.md#update_load_balancer) | **PUT** /load-balancer/{id} | Update a Load Balancer
[**update_load_balancer_service**](NetworkLoadBalancerApi.md#update_load_balancer_service) | **PUT** /load-balancer/{id}/service/{service_id} | Update a Load Balancer Service



## add_service_to_load_balancer

> models::Operation add_service_to_load_balancer(id, add_service_to_load_balancer_request)
Add a Load Balancer Service



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**add_service_to_load_balancer_request** | [**AddServiceToLoadBalancerRequest**](AddServiceToLoadBalancerRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_load_balancer

> models::Operation create_load_balancer(create_load_balancer_request)
Create a Load Balancer



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_load_balancer_request** | [**CreateLoadBalancerRequest**](CreateLoadBalancerRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_load_balancer

> models::Operation delete_load_balancer(id)
Delete a Load Balancer



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


## delete_load_balancer_service

> models::Operation delete_load_balancer_service(id, service_id)
Delete a Load Balancer Service



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_load_balancer

> models::LoadBalancer get_load_balancer(id)
Retrieve Load Balancer details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::LoadBalancer**](load-balancer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_load_balancer_service

> models::LoadBalancerService get_load_balancer_service(id, service_id)
Retrieve Load Balancer Service details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::LoadBalancerService**](load-balancer-service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_load_balancers

> models::ListLoadBalancers200Response list_load_balancers()
List Load Balancers



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListLoadBalancers200Response**](list_load_balancers_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_load_balancer_field

> models::Operation reset_load_balancer_field(id, field)
Reset a Load Balancer field to its default value



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


## reset_load_balancer_service_field

> models::Operation reset_load_balancer_service_field(id, service_id, field)
Reset a Load Balancer Service field to its default value



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_load_balancer

> models::Operation update_load_balancer(id, update_load_balancer_request)
Update a Load Balancer



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_load_balancer_request** | [**UpdateLoadBalancerRequest**](UpdateLoadBalancerRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_load_balancer_service

> models::Operation update_load_balancer_service(id, service_id, update_load_balancer_service_request)
Update a Load Balancer Service



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**service_id** | **uuid::Uuid** |  | [required] |
**update_load_balancer_service_request** | [**UpdateLoadBalancerServiceRequest**](UpdateLoadBalancerServiceRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

