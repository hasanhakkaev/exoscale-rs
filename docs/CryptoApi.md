# \CryptoApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decrypt**](CryptoApi.md#decrypt) | **POST** /kms-key/{id}/decrypt | [BETA] Decrypt
[**encrypt**](CryptoApi.md#encrypt) | **POST** /kms-key/{id}/encrypt | [BETA] Encrypt
[**generate_data_key**](CryptoApi.md#generate_data_key) | **POST** /kms-key/{id}/generate-data-key | [BETA] Generate Data Key
[**re_encrypt**](CryptoApi.md#re_encrypt) | **POST** /kms-key/{id}/re-encrypt | [BETA] Re-encrypt



## decrypt

> models::DecryptResponse decrypt(id, decrypt_request)
[BETA] Decrypt

Decrypt a ciphertext.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**decrypt_request** | [**DecryptRequest**](DecryptRequest.md) |  | [required] |

### Return type

[**models::DecryptResponse**](decrypt-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encrypt

> models::EncryptResponse encrypt(id, encrypt_request)
[BETA] Encrypt

Encrypt a plaintext.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**encrypt_request** | [**EncryptRequest**](EncryptRequest.md) |  | [required] |

### Return type

[**models::EncryptResponse**](encrypt-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_data_key

> models::GenerateDataKeyResponse generate_data_key(id, generate_data_key_request)
[BETA] Generate Data Key

Generate a Data Encryption Key from a given KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**generate_data_key_request** | [**GenerateDataKeyRequest**](GenerateDataKeyRequest.md) |  | [required] |

### Return type

[**models::GenerateDataKeyResponse**](generate-data-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## re_encrypt

> models::ReEncryptResponse re_encrypt(id, re_encrypt_request)
[BETA] Re-encrypt

Decrypts an existing ciphertext using its original key material and re-encrypts the underlying plaintext using a specified KMS key or the latest key material of the same KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**re_encrypt_request** | [**ReEncryptRequest**](ReEncryptRequest.md) |  | [required] |

### Return type

[**models::ReEncryptResponse**](re-encrypt-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

