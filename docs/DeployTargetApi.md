# \DeployTargetApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deploy_target**](DeployTargetApi.md#get_deploy_target) | **GET** /deploy-target/{id} | Retrieve Deploy Target details
[**list_deploy_targets**](DeployTargetApi.md#list_deploy_targets) | **GET** /deploy-target | List Deploy Targets



## get_deploy_target

> models::DeployTarget get_deploy_target(id)
Retrieve Deploy Target details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DeployTarget**](deploy-target.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deploy_targets

> models::ListDeployTargets200Response list_deploy_targets()
List Deploy Targets



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListDeployTargets200Response**](list_deploy_targets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

