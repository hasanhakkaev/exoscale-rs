# \SshKeyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_ssh_key**](SshKeyApi.md#delete_ssh_key) | **DELETE** /ssh-key/{name} | Delete a SSH key
[**get_ssh_key**](SshKeyApi.md#get_ssh_key) | **GET** /ssh-key/{name} | Retrieve SSH key details
[**list_ssh_keys**](SshKeyApi.md#list_ssh_keys) | **GET** /ssh-key | List SSH keys
[**register_ssh_key**](SshKeyApi.md#register_ssh_key) | **POST** /ssh-key | Import SSH key



## delete_ssh_key

> crate::models::Operation delete_ssh_key(name)
Delete a SSH key



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssh_key

> crate::models::SshKey get_ssh_key(name)
Retrieve SSH key details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::SshKey**](ssh-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ssh_keys

> crate::models::ListSshKeys200Response list_ssh_keys()
List SSH keys



### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ListSshKeys200Response**](list_ssh_keys_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_ssh_key

> crate::models::Operation register_ssh_key(register_ssh_key_request)
Import SSH key



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_ssh_key_request** | [**RegisterSshKeyRequest**](RegisterSshKeyRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

