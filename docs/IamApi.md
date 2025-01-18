# \IamApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](IamApi.md#create_api_key) | **POST** /api-key | Create a new API key
[**create_iam_role**](IamApi.md#create_iam_role) | **POST** /iam-role | Create IAM Role
[**create_user**](IamApi.md#create_user) | **POST** /user | Create a User
[**delete_api_key**](IamApi.md#delete_api_key) | **DELETE** /api-key/{id} | Delete an API key
[**delete_iam_role**](IamApi.md#delete_iam_role) | **DELETE** /iam-role/{id} | Delete IAM Role
[**delete_user**](IamApi.md#delete_user) | **DELETE** /user/{id} | Delete User
[**get_api_key**](IamApi.md#get_api_key) | **GET** /api-key/{id} | Get API key
[**get_iam_organization_policy**](IamApi.md#get_iam_organization_policy) | **GET** /iam-organization-policy | Retrieve IAM Organization Policy
[**get_iam_role**](IamApi.md#get_iam_role) | **GET** /iam-role/{id} | Retrieve IAM Role
[**list_api_keys**](IamApi.md#list_api_keys) | **GET** /api-key | List API keys
[**list_iam_roles**](IamApi.md#list_iam_roles) | **GET** /iam-role | List IAM Roles
[**list_users**](IamApi.md#list_users) | **GET** /user | List Users
[**update_iam_organization_policy**](IamApi.md#update_iam_organization_policy) | **PUT** /iam-organization-policy | Update IAM Organization Policy
[**update_iam_role**](IamApi.md#update_iam_role) | **PUT** /iam-role/{id} | Update IAM Role
[**update_iam_role_policy**](IamApi.md#update_iam_role_policy) | **PUT** /iam-role/{id}:policy | Update IAM Role Policy
[**update_user_role**](IamApi.md#update_user_role) | **PUT** /user/{id} | Update a User's IAM role



## create_api_key

> models::IamApiKeyCreated create_api_key(create_api_key_request)
Create a new API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::IamApiKeyCreated**](iam-api-key-created.md)

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


## create_user

> models::Operation create_user(create_user_request)
Create a User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_request** | [**CreateUserRequest**](CreateUserRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> models::Operation delete_api_key(id)
Delete an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
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


## delete_user

> models::Operation delete_user(id)
Delete User

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


## get_api_key

> models::IamApiKey get_api_key(id)
Get API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::IamApiKey**](iam-api-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## list_api_keys

> models::ListApiKeys200Response list_api_keys()
List API keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListApiKeys200Response**](list_api_keys_200_response.md)

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


## list_users

> models::ListUsers200Response list_users()
List Users

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListUsers200Response**](list_users_200_response.md)

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


## update_user_role

> models::Operation update_user_role(id, update_user_role_request)
Update a User's IAM role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_user_role_request** | [**UpdateUserRoleRequest**](UpdateUserRoleRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

