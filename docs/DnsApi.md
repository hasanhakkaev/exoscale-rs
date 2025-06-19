# \DnsApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dns_domain**](DnsApi.md#create_dns_domain) | **POST** /dns-domain | Create DNS domain
[**create_dns_domain_record**](DnsApi.md#create_dns_domain_record) | **POST** /dns-domain/{domain_id}/record | Create DNS domain record
[**delete_dns_domain**](DnsApi.md#delete_dns_domain) | **DELETE** /dns-domain/{id} | Delete DNS Domain
[**delete_dns_domain_record**](DnsApi.md#delete_dns_domain_record) | **DELETE** /dns-domain/{domain_id}/record/{record_id} | Delete DNS domain record
[**get_dns_domain**](DnsApi.md#get_dns_domain) | **GET** /dns-domain/{id} | Retrieve DNS domain details
[**get_dns_domain_record**](DnsApi.md#get_dns_domain_record) | **GET** /dns-domain/{domain_id}/record/{record_id} | Retrieve DNS domain record details
[**get_dns_domain_zone_file**](DnsApi.md#get_dns_domain_zone_file) | **GET** /dns-domain/{id}/zone | Retrieve DNS domain zone file
[**list_dns_domain_records**](DnsApi.md#list_dns_domain_records) | **GET** /dns-domain/{domain_id}/record | List DNS domain records
[**list_dns_domains**](DnsApi.md#list_dns_domains) | **GET** /dns-domain | List DNS domains
[**update_dns_domain_record**](DnsApi.md#update_dns_domain_record) | **PUT** /dns-domain/{domain_id}/record/{record_id} | Update DNS domain record



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


## create_dns_domain_record

> models::Operation create_dns_domain_record(domain_id, create_dns_domain_record_request)
Create DNS domain record



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**create_dns_domain_record_request** | [**CreateDnsDomainRecordRequest**](CreateDnsDomainRecordRequest.md) |  | [required] |

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


## delete_dns_domain_record

> models::Operation delete_dns_domain_record(domain_id, record_id)
Delete DNS domain record



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |

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


## get_dns_domain_record

> models::DnsDomainRecord get_dns_domain_record(domain_id, record_id)
Retrieve DNS domain record details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DnsDomainRecord**](dns-domain-record.md)

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


## list_dns_domain_records

> models::ListDnsDomainRecords200Response list_dns_domain_records(domain_id)
List DNS domain records



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListDnsDomainRecords200Response**](list_dns_domain_records_200_response.md)

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


## update_dns_domain_record

> models::Operation update_dns_domain_record(domain_id, record_id, update_dns_domain_record_request)
Update DNS domain record



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** |  | [required] |
**record_id** | **uuid::Uuid** |  | [required] |
**update_dns_domain_record_request** | [**UpdateDnsDomainRecordRequest**](UpdateDnsDomainRecordRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

