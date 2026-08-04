# \VpcManagementApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vpc**](VpcManagementApi.md#create_vpc) | **POST** /vpc | [BETA] Create a VPC
[**delete_vpc**](VpcManagementApi.md#delete_vpc) | **DELETE** /vpc/{id} | [BETA] Delete a VPC
[**get_vpc**](VpcManagementApi.md#get_vpc) | **GET** /vpc/{id} | [BETA] Retrieve VPC details
[**list_vpcs**](VpcManagementApi.md#list_vpcs) | **GET** /vpc | [BETA] List VPCs
[**update_vpc**](VpcManagementApi.md#update_vpc) | **PUT** /vpc/{id} | [BETA] Update a VPC



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

