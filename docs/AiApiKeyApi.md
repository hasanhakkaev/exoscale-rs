# \AiApiKeyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ai_api_key**](AiApiKeyApi.md#create_ai_api_key) | **POST** /ai/ai-api-key | Create AI API Key
[**delete_ai_api_key**](AiApiKeyApi.md#delete_ai_api_key) | **DELETE** /ai/ai-api-key/{id} | Delete AI API Key
[**get_ai_api_key**](AiApiKeyApi.md#get_ai_api_key) | **GET** /ai/ai-api-key/{id} | Get AI API Key
[**list_ai_api_keys**](AiApiKeyApi.md#list_ai_api_keys) | **GET** /ai/ai-api-key | List AI API Keys
[**rotate_ai_api_key**](AiApiKeyApi.md#rotate_ai_api_key) | **POST** /ai/ai-api-key/{id}/rotate | Rotate AI API Key
[**update_ai_api_key**](AiApiKeyApi.md#update_ai_api_key) | **PATCH** /ai/ai-api-key/{id} | Update AI API Key



## create_ai_api_key

> models::AiApiKeyWithValue create_ai_api_key(create_ai_api_key_request)
Create AI API Key

Create a new AI API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ai_api_key_request** | [**CreateAiApiKeyRequest**](CreateAiApiKeyRequest.md) |  | [required] |

### Return type

[**models::AiApiKeyWithValue**](ai-api-key-with-value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ai_api_key

> models::DeleteAiApiKey200Response delete_ai_api_key(id)
Delete AI API Key

Delete AI API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DeleteAiApiKey200Response**](delete_ai_api_key_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ai_api_key

> models::AiApiKey get_ai_api_key(id)
Get AI API Key

Get AI API key metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AiApiKey**](ai-api-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ai_api_keys

> models::ListAiApiKeysResponse list_ai_api_keys()
List AI API Keys

List AI API keys for an organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAiApiKeysResponse**](list-ai-api-keys-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_ai_api_key

> models::AiApiKeyWithValue rotate_ai_api_key(id)
Rotate AI API Key

Rotate AI API key value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::AiApiKeyWithValue**](ai-api-key-with-value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ai_api_key

> models::AiApiKey update_ai_api_key(id, update_ai_api_key_request)
Update AI API Key

Update AI API key name and/or scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_ai_api_key_request** | [**UpdateAiApiKeyRequest**](UpdateAiApiKeyRequest.md) |  | [required] |

### Return type

[**models::AiApiKey**](ai-api-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

