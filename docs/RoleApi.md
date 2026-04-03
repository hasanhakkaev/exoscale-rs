# \RoleApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assume_iam_role**](RoleApi.md#assume_iam_role) | **POST** /iam-role/{target_role_id}/assume | [BETA] Request generation of key/secret that allow caller to assume target role
[**create_iam_role**](RoleApi.md#create_iam_role) | **POST** /iam-role | Create IAM Role
[**delete_iam_role**](RoleApi.md#delete_iam_role) | **DELETE** /iam-role/{id} | Delete IAM Role
[**get_iam_role**](RoleApi.md#get_iam_role) | **GET** /iam-role/{id} | Retrieve IAM Role
[**list_iam_roles**](RoleApi.md#list_iam_roles) | **GET** /iam-role | List IAM Roles
[**update_iam_role**](RoleApi.md#update_iam_role) | **PUT** /iam-role/{id} | Update IAM Role
[**update_iam_role_assume_policy**](RoleApi.md#update_iam_role_assume_policy) | **PUT** /iam-role/{id}:assume-role-policy | Update IAM Assume role Policy
[**update_iam_role_policy**](RoleApi.md#update_iam_role_policy) | **PUT** /iam-role/{id}:policy | Update IAM Role Policy



## assume_iam_role

> models::AssumeIamRole200Response assume_iam_role(target_role_id, assume_iam_role_request)
[BETA] Request generation of key/secret that allow caller to assume target role

[BETA] Request generation of key/secret that allow caller to assume target role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target_role_id** | **uuid::Uuid** |  | [required] |
**assume_iam_role_request** | [**AssumeIamRoleRequest**](AssumeIamRoleRequest.md) |  | [required] |

### Return type

[**models::AssumeIamRole200Response**](assume_iam_role_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_iam_role

> models::Operation create_iam_role(create_iam_role_request)
Create IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_iam_role_request** | [**CreateIamRoleRequest**](CreateIamRoleRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_iam_role

> models::Operation delete_iam_role(id)
Delete IAM Role



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


## get_iam_role

> models::IamRole get_iam_role(id)
Retrieve IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::IamRole**](iam-role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_iam_roles

> models::ListIamRoles200Response list_iam_roles()
List IAM Roles



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListIamRoles200Response**](list_iam_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_role

> models::Operation update_iam_role(id, update_iam_role_request)
Update IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_iam_role_request** | [**UpdateIamRoleRequest**](UpdateIamRoleRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_role_assume_policy

> models::Operation update_iam_role_assume_policy(id, iam_policy)
Update IAM Assume role Policy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**iam_policy** | [**IamPolicy**](IamPolicy.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_role_policy

> models::Operation update_iam_role_policy(id, iam_policy)
Update IAM Role Policy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**iam_policy** | [**IamPolicy**](IamPolicy.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

