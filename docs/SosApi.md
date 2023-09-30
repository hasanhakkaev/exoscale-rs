# \SosApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sos_presigned_url**](SosApi.md#get_sos_presigned_url) | **GET** /sos/{bucket}/presigned-url | Retrieve Presigned Download URL for SOS object
[**list_sos_buckets_usage**](SosApi.md#list_sos_buckets_usage) | **GET** /sos-buckets-usage | List SOS Buckets Usage



## get_sos_presigned_url

> crate::models::GetSosPresignedUrl200Response get_sos_presigned_url(bucket, key)
Retrieve Presigned Download URL for SOS object

Generates Presigned Download URL for SOS object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bucket** | **String** |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**crate::models::GetSosPresignedUrl200Response**](get_sos_presigned_url_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sos_buckets_usage

> crate::models::ListSosBucketsUsage200Response list_sos_buckets_usage()
List SOS Buckets Usage



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListSosBucketsUsage200Response**](list_sos_buckets_usage_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

