# \TemplateApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_template**](TemplateApi.md#copy_template) | **POST** /template/{id} | Copy a Template from a zone to another
[**delete_template**](TemplateApi.md#delete_template) | **DELETE** /template/{id} | Delete a Template
[**get_template**](TemplateApi.md#get_template) | **GET** /template/{id} | Retrieve Template details
[**list_templates**](TemplateApi.md#list_templates) | **GET** /template | List Templates
[**promote_snapshot_to_template**](TemplateApi.md#promote_snapshot_to_template) | **POST** /snapshot/{id}:promote | Promote a Snapshot to a Template
[**register_template**](TemplateApi.md#register_template) | **POST** /template | Register a Template
[**update_template**](TemplateApi.md#update_template) | **PUT** /template/{id} | Update template attributes



## copy_template

> crate::models::Operation copy_template(id, copy_template_request)
Copy a Template from a zone to another



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**copy_template_request** | [**CopyTemplateRequest**](CopyTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_template

> crate::models::Operation delete_template(id)
Delete a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template

> crate::models::Template get_template(id)
Retrieve Template details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> crate::models::ListTemplates200Response list_templates(visibility, family)
List Templates



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility** | Option<**String**> |  |  |
**family** | Option<**String**> |  |  |

### Return type

[**crate::models::ListTemplates200Response**](list_templates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## promote_snapshot_to_template

> crate::models::Operation promote_snapshot_to_template(id, promote_snapshot_to_template_request)
Promote a Snapshot to a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**promote_snapshot_to_template_request** | [**PromoteSnapshotToTemplateRequest**](PromoteSnapshotToTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_template

> crate::models::Operation register_template(register_template_request)
Register a Template



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_template_request** | [**RegisterTemplateRequest**](RegisterTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_template

> crate::models::Operation update_template(id, update_template_request)
Update template attributes



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_template_request** | [**UpdateTemplateRequest**](UpdateTemplateRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

