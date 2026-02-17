# \OrganizationPolicyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_iam_organization_policy**](OrganizationPolicyApi.md#get_iam_organization_policy) | **GET** /iam-organization-policy | Retrieve IAM Organization Policy
[**reset_iam_organization_policy**](OrganizationPolicyApi.md#reset_iam_organization_policy) | **POST** /iam-organization-policy:reset | Reset IAM Organization Policy
[**update_iam_organization_policy**](OrganizationPolicyApi.md#update_iam_organization_policy) | **PUT** /iam-organization-policy | Update IAM Organization Policy



## get_iam_organization_policy

> models::IamPolicy get_iam_organization_policy()
Retrieve IAM Organization Policy



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IamPolicy**](iam-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_iam_organization_policy

> models::Operation reset_iam_organization_policy()
Reset IAM Organization Policy



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_organization_policy

> models::Operation update_iam_organization_policy(iam_policy)
Update IAM Organization Policy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_policy** | [**IamPolicy**](IamPolicy.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

