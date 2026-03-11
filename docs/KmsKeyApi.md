# \KmsKeyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_kms_key_deletion**](KmsKeyApi.md#cancel_kms_key_deletion) | **POST** /kms-key/{id}/cancel-deletion | [Beta] Cancel KMS Key Deletion
[**create_kms_key**](KmsKeyApi.md#create_kms_key) | **POST** /kms-key | [BETA] Create KMS Key
[**disable_kms_key**](KmsKeyApi.md#disable_kms_key) | **POST** /kms-key/{id}/disable | [BETA] Disable KMS Key
[**disable_kms_key_rotation**](KmsKeyApi.md#disable_kms_key_rotation) | **POST** /kms-key/{id}/disable-key-rotation | [BETA] Disable Key Rotation
[**enable_kms_key**](KmsKeyApi.md#enable_kms_key) | **POST** /kms-key/{id}/enable | [BETA] Enable KMS Key
[**enable_kms_key_rotation**](KmsKeyApi.md#enable_kms_key_rotation) | **POST** /kms-key/{id}/enable-key-rotation | [BETA] Enable Key Rotation
[**get_kms_key**](KmsKeyApi.md#get_kms_key) | **GET** /kms-key/{id} | [BETA] Get KMS Key
[**list_kms_key_rotations**](KmsKeyApi.md#list_kms_key_rotations) | **GET** /kms-key/{id}/list-key-rotations | [BETA] List KMS Key Rotations
[**list_kms_keys**](KmsKeyApi.md#list_kms_keys) | **GET** /kms-key | [BETA] List KMS Keys
[**replicate_kms_key**](KmsKeyApi.md#replicate_kms_key) | **POST** /kms-key/{id}/replicate | [BETA] Replicate KMS Key
[**rotate_kms_key**](KmsKeyApi.md#rotate_kms_key) | **POST** /kms-key/{id}/rotate | [BETA] Rotate Key
[**schedule_kms_key_deletion**](KmsKeyApi.md#schedule_kms_key_deletion) | **POST** /kms-key/{id}/schedule-deletion | [BETA] Schedule KMS Key Deletion



## cancel_kms_key_deletion

> cancel_kms_key_deletion(id)
[Beta] Cancel KMS Key Deletion

Cancel the scheduled deletion of a KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_kms_key

> models::CreateKmsKeyResponse create_kms_key(create_kms_key_request)
[BETA] Create KMS Key

Create a KMS Key in a given zone with a given name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_kms_key_request** | [**CreateKmsKeyRequest**](CreateKmsKeyRequest.md) |  | [required] |

### Return type

[**models::CreateKmsKeyResponse**](create-kms-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_kms_key

> models::SuccessResponse disable_kms_key(id)
[BETA] Disable KMS Key

Disable a KMS Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SuccessResponse**](success-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_kms_key_rotation

> models::DisableKmsKeyRotationResponse disable_kms_key_rotation(id, disable_kms_key_rotation_request)
[BETA] Disable Key Rotation

Disable the periodic rotation of a KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**disable_kms_key_rotation_request** | [**DisableKmsKeyRotationRequest**](DisableKmsKeyRotationRequest.md) |  | [required] |

### Return type

[**models::DisableKmsKeyRotationResponse**](disable-kms-key-rotation-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_kms_key

> models::SuccessResponse enable_kms_key(id)
[BETA] Enable KMS Key

Enable a KMS Key\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SuccessResponse**](success-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_kms_key_rotation

> models::EnableKmsKeyRotationResponse enable_kms_key_rotation(id, enable_kms_key_rotation_request)
[BETA] Enable Key Rotation

Enable the periodic rotation of a KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**enable_kms_key_rotation_request** | [**EnableKmsKeyRotationRequest**](EnableKmsKeyRotationRequest.md) |  | [required] |

### Return type

[**models::EnableKmsKeyRotationResponse**](enable-kms-key-rotation-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_kms_key

> models::GetKmsKeyResponse get_kms_key(id)
[BETA] Get KMS Key

Retrieve KMS Key details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::GetKmsKeyResponse**](get-kms-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kms_key_rotations

> models::ListKmsKeyRotationsResponse list_kms_key_rotations(id)
[BETA] List KMS Key Rotations

List all the key material versions of a KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ListKmsKeyRotationsResponse**](list-kms-key-rotations-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_kms_keys

> models::ListKmsKeysResponse list_kms_keys()
[BETA] List KMS Keys

List KMS Keys details for an organization in a given zone.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListKmsKeysResponse**](list-kms-keys-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replicate_kms_key

> models::Operation replicate_kms_key(id, replicate_kms_key_request)
[BETA] Replicate KMS Key

Replicate a KMS key to a target zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**replicate_kms_key_request** | [**ReplicateKmsKeyRequest**](ReplicateKmsKeyRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_kms_key

> models::RotateKmsKeyResponse rotate_kms_key(id)
[BETA] Rotate Key

Perform a manual rotation of the key material for a symmetric key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RotateKmsKeyResponse**](rotate-kms-key-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_kms_key_deletion

> schedule_kms_key_deletion(id, schedule_kms_key_deletion_request)
[BETA] Schedule KMS Key Deletion

Schedule a KMS key for deletion after a delay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**schedule_kms_key_deletion_request** | [**ScheduleKmsKeyDeletionRequest**](ScheduleKmsKeyDeletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

