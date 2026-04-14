# \DeploymentApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deployment**](DeploymentApi.md#create_deployment) | **POST** /ai/deployment | Create Deployment
[**delete_deployment**](DeploymentApi.md#delete_deployment) | **DELETE** /ai/deployment/{id} | Delete Deployment
[**get_deployment**](DeploymentApi.md#get_deployment) | **GET** /ai/deployment/{id} | Get Deployment
[**get_deployment_logs**](DeploymentApi.md#get_deployment_logs) | **GET** /ai/deployment/{id}/logs | Get Deployment Logs
[**get_inference_engine_help**](DeploymentApi.md#get_inference_engine_help) | **GET** /ai/help/inference-engine-parameters | Get inference-engine Help
[**list_ai_instance_types**](DeploymentApi.md#list_ai_instance_types) | **GET** /ai/instance-type | List Instance Types
[**list_deployments**](DeploymentApi.md#list_deployments) | **GET** /ai/deployment | List Deployments
[**reveal_deployment_api_key**](DeploymentApi.md#reveal_deployment_api_key) | **GET** /ai/deployment/{id}/api-key | Reveal Deployment API Key
[**scale_deployment**](DeploymentApi.md#scale_deployment) | **POST** /ai/deployment/{id}/scale | Scale Deployment
[**update_deployment**](DeploymentApi.md#update_deployment) | **PATCH** /ai/deployment/{id} | 



## create_deployment

> models::Operation create_deployment(create_deployment_request)
Create Deployment

Deploy a model on an inference server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_deployment_request** | [**CreateDeploymentRequest**](CreateDeploymentRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deployment

> models::Operation delete_deployment(id)
Delete Deployment

Delete Deployment

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


## get_deployment

> models::GetDeploymentResponse get_deployment(id)
Get Deployment

Get Deployment details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetDeploymentResponse**](get-deployment-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_logs

> models::GetDeploymentLogsResponse get_deployment_logs(id, stream, tail)
Get Deployment Logs

Return logs for the vLLM deployment (deploy/<release-name>--deployment-vllm). Optional ?stream=true to request streaming (may not be supported).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**stream** | Option<**bool**> |  |  |
**tail** | Option<**i64**> |  |  |

### Return type

[**models::GetDeploymentLogsResponse**](get-deployment-logs-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inference_engine_help

> models::GetInferenceEngineHelpResponse get_inference_engine_help(version)
Get inference-engine Help

Get list of allowed inference engine parameters with their descriptions and allowed values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | Option<**String**> |  |  |

### Return type

[**models::GetInferenceEngineHelpResponse**](get-inference-engine-help-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ai_instance_types

> models::ListAiInstanceTypesResponse list_ai_instance_types()
List Instance Types

List available instance types with authorization status based on GPU availability

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAiInstanceTypesResponse**](list-ai-instance-types-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deployments

> models::ListDeploymentsResponse list_deployments(visibility)
List Deployments

List Deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility** | Option<**String**> |  |  |

### Return type

[**models::ListDeploymentsResponse**](list-deployments-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reveal_deployment_api_key

> models::RevealDeploymentApiKeyResponse reveal_deployment_api_key(id)
Reveal Deployment API Key

Get Deployment API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RevealDeploymentApiKeyResponse**](reveal-deployment-api-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_deployment

> models::Operation scale_deployment(id, scale_deployment_request)
Scale Deployment

Scale Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**scale_deployment_request** | [**ScaleDeploymentRequest**](ScaleDeploymentRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deployment

> models::Operation update_deployment(id, update_deployment_request)


Update AI deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_deployment_request** | [**UpdateDeploymentRequest**](UpdateDeploymentRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

