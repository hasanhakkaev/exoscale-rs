# \IamApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_access_key**](IamApi.md#create_access_key) | **POST** /access-key | Create a legacy IAM Access Key
[**create_api_key**](IamApi.md#create_api_key) | **POST** /api-key | Create a new API key
[**create_iam_role**](IamApi.md#create_iam_role) | **POST** /iam-role | Create IAM Role
[**delete_api_key**](IamApi.md#delete_api_key) | **DELETE** /api-key/{id} | Delete an API key
[**delete_iam_role**](IamApi.md#delete_iam_role) | **DELETE** /iam-role/{id} | Delete IAM Role
[**get_access_key**](IamApi.md#get_access_key) | **GET** /access-key/{key} | Retrieve IAM Access Key details
[**get_api_key**](IamApi.md#get_api_key) | **GET** /api-key/{id} | Get API key
[**get_iam_organization_policy**](IamApi.md#get_iam_organization_policy) | **GET** /iam-organization-policy | Retrieve IAM Organization Policy
[**get_iam_role**](IamApi.md#get_iam_role) | **GET** /iam-role/{id} | Retrieve IAM Role
[**list_access_key_known_operations**](IamApi.md#list_access_key_known_operations) | **GET** /access-key-known-operations | Retrieve all known available IAM Access Key operations and associated tags
[**list_access_key_operations**](IamApi.md#list_access_key_operations) | **GET** /access-key-operations | Retrieve IAM Access Key operations and associated tags for the signing key
[**list_access_keys**](IamApi.md#list_access_keys) | **GET** /access-key | List IAM Access Keys
[**list_api_keys**](IamApi.md#list_api_keys) | **GET** /api-key | List API keys
[**list_iam_roles**](IamApi.md#list_iam_roles) | **GET** /iam-role | List IAM Roles
[**revoke_access_key**](IamApi.md#revoke_access_key) | **DELETE** /access-key/{key} | Revoke an IAM Access Key
[**update_iam_organization_policy**](IamApi.md#update_iam_organization_policy) | **PUT** /iam-organization-policy | Update IAM Organization Policy
[**update_iam_role**](IamApi.md#update_iam_role) | **PUT** /iam-role/{id} | Update IAM Role
[**update_iam_role_policy**](IamApi.md#update_iam_role_policy) | **PUT** /iam-role/{id}:policy | Update IAM Role Policy



## create_access_key

> crate::models::AccessKey create_access_key(create_access_key_request)
Create a legacy IAM Access Key

This operation creates a legacy IAM Access Key, to create a key for use with IAM roles use the api-key endpoint.The corresponding secret is only available in the response returned by this operation, the caller must take care of storing it safely as there is no other way to retrieve it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_access_key_request** | [**CreateAccessKeyRequest**](CreateAccessKeyRequest.md) |  | [required] |

### Return type

[**crate::models::AccessKey**](access-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_api_key

> crate::models::IamApiKeyCreated create_api_key(create_api_key_request)
Create a new API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**crate::models::IamApiKeyCreated**](iam-api-key-created.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_iam_role

> crate::models::Operation create_iam_role(create_iam_role_request)
Create IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_iam_role_request** | [**CreateIamRoleRequest**](CreateIamRoleRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> crate::models::Operation delete_api_key(id)
Delete an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_iam_role

> crate::models::Operation delete_iam_role(id)
Delete IAM Role



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


## get_access_key

> crate::models::AccessKey get_access_key(key)
Retrieve IAM Access Key details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**crate::models::AccessKey**](access-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_key

> crate::models::IamApiKey get_api_key(id)
Get API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::IamApiKey**](iam-api-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_iam_organization_policy

> crate::models::IamPolicy get_iam_organization_policy()
Retrieve IAM Organization Policy



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IamPolicy**](iam-policy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_iam_role

> crate::models::IamRole get_iam_role(id)
Retrieve IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::IamRole**](iam-role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_key_known_operations

> crate::models::ListAccessKeyKnownOperations200Response list_access_key_known_operations()
Retrieve all known available IAM Access Key operations and associated tags



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeyKnownOperations200Response**](list_access_key_known_operations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_key_operations

> crate::models::ListAccessKeyKnownOperations200Response list_access_key_operations()
Retrieve IAM Access Key operations and associated tags for the signing key



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeyKnownOperations200Response**](list_access_key_known_operations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_access_keys

> crate::models::ListAccessKeys200Response list_access_keys()
List IAM Access Keys



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListAccessKeys200Response**](list_access_keys_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys

> crate::models::ListApiKeys200Response list_api_keys()
List API keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListApiKeys200Response**](list_api_keys_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_iam_roles

> crate::models::ListIamRoles200Response list_iam_roles()
List IAM Roles



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListIamRoles200Response**](list_iam_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_access_key

> crate::models::Operation revoke_access_key(key)
Revoke an IAM Access Key

This operation revokes the specified IAM Access Key. Access Keys created by the revoked Access Key will not be revoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_organization_policy

> crate::models::Operation update_iam_organization_policy(iam_policy)
Update IAM Organization Policy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_policy** | [**IamPolicy**](IamPolicy.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_role

> crate::models::Operation update_iam_role(id, update_iam_role_request)
Update IAM Role



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_iam_role_request** | [**UpdateIamRoleRequest**](UpdateIamRoleRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iam_role_policy

> crate::models::Operation update_iam_role_policy(id, iam_policy)
Update IAM Role Policy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**iam_policy** | [**IamPolicy**](IamPolicy.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

