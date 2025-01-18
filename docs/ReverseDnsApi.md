# \ReverseDnsApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_reverse_dns_elastic_ip**](ReverseDnsApi.md#delete_reverse_dns_elastic_ip) | **DELETE** /reverse-dns/elastic-ip/{id} | Delete the PTR DNS record for an elastic IP
[**delete_reverse_dns_instance**](ReverseDnsApi.md#delete_reverse_dns_instance) | **DELETE** /reverse-dns/instance/{id} | Delete the PTR DNS record for an instance
[**get_reverse_dns_elastic_ip**](ReverseDnsApi.md#get_reverse_dns_elastic_ip) | **GET** /reverse-dns/elastic-ip/{id} | Query the PTR DNS records for an elastic IP
[**get_reverse_dns_instance**](ReverseDnsApi.md#get_reverse_dns_instance) | **GET** /reverse-dns/instance/{id} | Query the PTR DNS records for an instance
[**update_reverse_dns_elastic_ip**](ReverseDnsApi.md#update_reverse_dns_elastic_ip) | **POST** /reverse-dns/elastic-ip/{id} | Update/Create the PTR DNS record for an elastic IP
[**update_reverse_dns_instance**](ReverseDnsApi.md#update_reverse_dns_instance) | **POST** /reverse-dns/instance/{id} | Update/Create the PTR DNS record for an instance



## delete_reverse_dns_elastic_ip

> models::Operation delete_reverse_dns_elastic_ip(id)
Delete the PTR DNS record for an elastic IP



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


## delete_reverse_dns_instance

> models::Operation delete_reverse_dns_instance(id)
Delete the PTR DNS record for an instance



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


## get_reverse_dns_elastic_ip

> models::ReverseDnsRecord get_reverse_dns_elastic_ip(id)
Query the PTR DNS records for an elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ReverseDnsRecord**](reverse-dns-record.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_reverse_dns_instance

> models::ReverseDnsRecord get_reverse_dns_instance(id)
Query the PTR DNS records for an instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ReverseDnsRecord**](reverse-dns-record.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reverse_dns_elastic_ip

> models::Operation update_reverse_dns_elastic_ip(id, update_reverse_dns_elastic_ip_request)
Update/Create the PTR DNS record for an elastic IP



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_reverse_dns_elastic_ip_request** | [**UpdateReverseDnsElasticIpRequest**](UpdateReverseDnsElasticIpRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reverse_dns_instance

> models::Operation update_reverse_dns_instance(id, update_reverse_dns_elastic_ip_request)
Update/Create the PTR DNS record for an instance



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_reverse_dns_elastic_ip_request** | [**UpdateReverseDnsElasticIpRequest**](UpdateReverseDnsElasticIpRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

