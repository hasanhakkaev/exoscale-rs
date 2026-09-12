# \VpcRouteApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_route**](VpcRouteApi.md#create_route) | **POST** /vpc/{vpc_id}/subnet/{subnet_id}/route | [BETA] Create a route
[**delete_route**](VpcRouteApi.md#delete_route) | **DELETE** /vpc/{vpc_id}/subnet/{subnet_id}/route/{id} | [BETA] Delete a route
[**list_routes**](VpcRouteApi.md#list_routes) | **GET** /vpc/{vpc_id}/subnet/{subnet_id}/route | [BETA] List Subnet routes
[**list_vpc_routes**](VpcRouteApi.md#list_vpc_routes) | **GET** /vpc/{vpc_id}/route | [BETA] List VPC routes



## create_route

> models::Route create_route(vpc_id, subnet_id, create_route_request)
[BETA] Create a route



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**subnet_id** | **uuid::Uuid** |  | [required] |
**create_route_request** | [**CreateRouteRequest**](CreateRouteRequest.md) |  | [required] |

### Return type

[**models::Route**](route.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_route

> serde_json::Value delete_route(vpc_id, subnet_id, id)
[BETA] Delete a route



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**subnet_id** | **uuid::Uuid** |  | [required] |
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_routes

> models::ListVpcRoutes200Response list_routes(vpc_id, subnet_id)
[BETA] List Subnet routes



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |
**subnet_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListVpcRoutes200Response**](list_vpc_routes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_vpc_routes

> models::ListVpcRoutes200Response list_vpc_routes(vpc_id)
[BETA] List VPC routes



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vpc_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListVpcRoutes200Response**](list_vpc_routes_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

