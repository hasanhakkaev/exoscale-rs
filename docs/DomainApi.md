# \DomainApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dns_domain**](DomainApi.md#create_dns_domain) | **POST** /dns-domain | Create DNS domain
[**delete_dns_domain**](DomainApi.md#delete_dns_domain) | **DELETE** /dns-domain/{id} | Delete DNS Domain
[**get_dns_domain**](DomainApi.md#get_dns_domain) | **GET** /dns-domain/{id} | Retrieve DNS domain details
[**get_dns_domain_zone_file**](DomainApi.md#get_dns_domain_zone_file) | **GET** /dns-domain/{id}/zone | Retrieve DNS domain zone file
[**list_dns_domains**](DomainApi.md#list_dns_domains) | **GET** /dns-domain | List DNS domains



## create_dns_domain

> models::Operation create_dns_domain(create_dns_domain_request)
Create DNS domain



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dns_domain_request** | [**CreateDnsDomainRequest**](CreateDnsDomainRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_domain

> models::Operation delete_dns_domain(id)
Delete DNS Domain



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


## get_dns_domain

> models::DnsDomain get_dns_domain(id)
Retrieve DNS domain details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DnsDomain**](dns-domain.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_domain_zone_file

> models::GetDnsDomainZoneFile200Response get_dns_domain_zone_file(id)
Retrieve DNS domain zone file



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetDnsDomainZoneFile200Response**](get_dns_domain_zone_file_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dns_domains

> models::ListDnsDomains200Response list_dns_domains()
List DNS domains



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDnsDomains200Response**](list_dns_domains_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

