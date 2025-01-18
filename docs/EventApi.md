# \EventApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

 Method                                     | HTTP request   | Description 
--------------------------------------------|----------------|-------------
 [**list_events**](EventApi.md#list_events) | **GET** /event | List Events 

## list_events

> Vec<models::Event> list_events(from, to)
> List Events

Retrieve Mutation Events for a given date range. Defaults to retrieving Events for the past 24 hours. Both a `from` and
`to` arguments can be specified to filter Events over a specific period. Events will be the most descriptive possible
but not all fields are mandatory

### Parameters

 Name     | Type               | Description | Required | Notes 
----------|--------------------|-------------|----------|-------
 **from** | Option<**String**> |             |          |
 **to**   | Option<**String**> |             |          |

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

