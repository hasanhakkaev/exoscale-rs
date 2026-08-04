# \DefaultApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_impact_estimate**](DefaultApi.md#get_impact_estimate) | **POST** /environmental-impact/estimate | Return an estimate of the impact of a given usage
[**get_impact_report**](DefaultApi.md#get_impact_report) | **GET** /environmental-impact/report | Return an environmental impact report for the given period



## get_impact_estimate

> models::GetImpactEstimate200Response get_impact_estimate(get_impact_estimate_request)
Return an estimate of the impact of a given usage



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_impact_estimate_request** | [**GetImpactEstimateRequest**](GetImpactEstimateRequest.md) |  | [required] |

### Return type

[**models::GetImpactEstimate200Response**](get_impact_estimate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_impact_report

> models::ImpactBreakdown get_impact_report(from, to)
Return an environmental impact report for the given period



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |

### Return type

[**models::ImpactBreakdown**](impact-breakdown.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

