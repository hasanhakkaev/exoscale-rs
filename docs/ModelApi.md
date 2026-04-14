# \ModelApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_model**](ModelApi.md#create_model) | **POST** /ai/model | Create Model
[**delete_model**](ModelApi.md#delete_model) | **DELETE** /ai/model/{id} | Delete Model
[**get_model**](ModelApi.md#get_model) | **GET** /ai/model/{id} | Get Model
[**list_models**](ModelApi.md#list_models) | **GET** /ai/model | List Models



## create_model

> models::Operation create_model(create_model_request)
Create Model

Model files will be downloaded from Huggingface.  Name must be the exact name of the model on huggingface (ex: openai/gpt-oss-120b or ggml-org/gpt-oss-120b-GGUF).  If the model is under a license then you must provide a Huggingface access token for an account that signed the license agreement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_model_request** | [**CreateModelRequest**](CreateModelRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_model

> models::Operation delete_model(id)
Delete Model

Delete Model

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


## get_model

> models::GetModelResponse get_model(id)
Get Model

Get Model details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetModelResponse**](get-model-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_models

> models::ListModelsResponse list_models(visibility)
List Models

List Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility** | Option<**String**> |  |  |

### Return type

[**models::ListModelsResponse**](list-models-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

