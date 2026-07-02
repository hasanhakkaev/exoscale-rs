# \VpcApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**attach_instance_to_subnet**](VpcApi.md#attach_instance_to_subnet) | **PUT** /vpc/{vpc_id}/subnet/{subnet_id}/attach | [BETA] Attach a Compute instance to a Subnet
[**create_route**](VpcApi.md#create_route) | **POST** /vpc/{vpc_id}/subnet/{subnet_id}/route | [BETA] Create a route
[**create_subnet**](VpcApi.md#create_subnet) | **POST** /vpc/{vpc_id}/subnet | [BETA] Create a Subnet
[**create_vpc**](VpcApi.md#create_vpc) | **POST** /vpc | [BETA] Create a VPC
[**delete_route**](VpcApi.md#delete_route) | **DELETE** /vpc/{vpc_id}/subnet/{subnet_id}/route/{id} | [BETA] Delete a route
[**delete_subnet**](VpcApi.md#delete_subnet) | **DELETE** /vpc/{vpc_id}/subnet/{id} | [BETA] Delete a Subnet
[**delete_vpc**](VpcApi.md#delete_vpc) | **DELETE** /vpc/{id} | [BETA] Delete a VPC
[**detach_instance_from_subnet**](VpcApi.md#detach_instance_from_subnet) | **PUT** /vpc/{vpc_id}/subnet/{subnet_id}/detach | [BETA] Detach a Compute instance from a Subnet
[**get_subnet**](VpcApi.md#get_subnet) | **GET** /vpc/{vpc_id}/subnet/{id} | [BETA] Retrieve Subnet details
[**get_vpc**](VpcApi.md#get_vpc) | **GET** /vpc/{id} | [BETA] Retrieve VPC details
[**list_routes**](VpcApi.md#list_routes) | **GET** /vpc/{vpc_id}/subnet/{subnet_id}/route | [BETA] List Subnet routes
[**list_subnets**](VpcApi.md#list_subnets) | **GET** /vpc/{vpc_id}/subnet | [BETA] List Subnets
[**list_vpc_routes**](VpcApi.md#list_vpc_routes) | **GET** /vpc/{vpc_id}/route | [BETA] List VPC routes
[**list_vpcs**](VpcApi.md#list_vpcs) | **GET** /vpc | [BETA] List VPCs
[**update_subnet**](VpcApi.md#update_subnet) | **PUT** /vpc/{vpc_id}/subnet/{id} | [BETA] Update a Subnet
[**update_vpc**](VpcApi.md#update_vpc) | **PUT** /vpc/{id} | [BETA] Update a VPC



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


## create_vpc

> models::Operation create_vpc(create_vpc_request)
[BETA] Create a VPC



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vpc_request** | [**CreateVpcRequest**](CreateVpcRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

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


## delete_vpc

> serde_json::Value delete_vpc(id)
[BETA] Delete a VPC



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


## detach_instance_from_subnet

> models::Operation detach_instance_from_subnet(vpc_id, subnet_id, attach_instance_to_subnet_request)
[BETA] Detach a Compute instance from a Subnet



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


## get_vpc

> models::Vpc get_vpc(id)
[BETA] Retrieve VPC details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Vpc**](vpc.md)

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


## list_vpcs

> models::ListVpcs200Response list_vpcs()
[BETA] List VPCs



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListVpcs200Response**](list_vpcs_200_response.md)

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


## update_vpc

> models::Vpc update_vpc(id, update_vpc_request)
[BETA] Update a VPC



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_vpc_request** | [**UpdateVpcRequest**](UpdateVpcRequest.md) |  | [required] |

### Return type

[**models::Vpc**](vpc.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

