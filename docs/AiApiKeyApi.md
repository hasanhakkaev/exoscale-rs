# \AiApiKeyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ai_api_key**](AiApiKeyApi.md#create_ai_api_key) | **POST** /ai/api-key | Create AI API Key
[**delete_ai_api_key**](AiApiKeyApi.md#delete_ai_api_key) | **DELETE** /ai/api-key/{id} | Delete AI API Key
[**get_ai_api_key**](AiApiKeyApi.md#get_ai_api_key) | **GET** /ai/api-key/{id} | Get AI API Key
[**get_user_org_consumption_quota**](AiApiKeyApi.md#get_user_org_consumption_quota) | **GET** /ai/quota | Get Organization Consumption Quota
[**list_ai_api_keys**](AiApiKeyApi.md#list_ai_api_keys) | **GET** /ai/api-key | List AI API Keys
[**reveal_ai_api_key**](AiApiKeyApi.md#reveal_ai_api_key) | **GET** /ai/api-key/{id}/reveal | Reveal AI API Key
[**rotate_ai_api_key**](AiApiKeyApi.md#rotate_ai_api_key) | **POST** /ai/api-key/{id}/rotate | Rotate AI API Key
[**update_ai_api_key**](AiApiKeyApi.md#update_ai_api_key) | **PATCH** /ai/api-key/{id} | Update AI API Key



## create_ai_api_key

> models::CreateAiApiKeyResponse create_ai_api_key(create_ai_api_key_request)
Create AI API Key

Create a new AI API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_ai_api_key_request** | [**CreateAiApiKeyRequest**](CreateAiApiKeyRequest.md) |  | [required] |

### Return type

[**models::CreateAiApiKeyResponse**](create-ai-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ai_api_key

> models::Operation delete_ai_api_key(id)
Delete AI API Key

Delete AI API key

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


## get_ai_api_key

> models::GetAiApiKeyResponse get_ai_api_key(id)
Get AI API Key

Get AI API key metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetAiApiKeyResponse**](get-ai-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_org_consumption_quota

> models::OrgConsumptionQuotaResponse get_user_org_consumption_quota()
Get Organization Consumption Quota

Get per-org Unit Of Measurement (UOM) consumption quota (UOM/min). Null means unlimited. UOM represents weighted units across different AI workloads (e.g., tokens for LLMs, minutes for TTS, pages for OCR).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OrgConsumptionQuotaResponse**](org-consumption-quota-response.md)

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


## reveal_ai_api_key

> models::RevealAiApiKeyResponse reveal_ai_api_key(id)
Reveal AI API Key

Reveal AI API key plaintext value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RevealAiApiKeyResponse**](reveal-ai-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_ai_api_key

> models::RotateAiApiKeyResponse rotate_ai_api_key(id)
Rotate AI API Key

Rotate AI API key value

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RotateAiApiKeyResponse**](rotate-ai-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ai_api_key

> models::UpdateAiApiKeyResponse update_ai_api_key(id, update_ai_api_key_request)
Update AI API Key

Update AI API key name and/or scope

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_ai_api_key_request** | [**UpdateAiApiKeyRequest**](UpdateAiApiKeyRequest.md) |  | [required] |

### Return type

[**models::UpdateAiApiKeyResponse**](update-ai-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

