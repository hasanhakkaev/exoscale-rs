# \KmsKeyApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_kms_key_deletion**](KmsKeyApi.md#cancel_kms_key_deletion) | **POST** /kms-key/{id}/cancel-deletion | Cancel KMS Key Deletion
[**create_kms_key**](KmsKeyApi.md#create_kms_key) | **POST** /kms-key | Create KMS Key
[**disable_kms_key**](KmsKeyApi.md#disable_kms_key) | **POST** /kms-key/{id}/disable | Disable KMS Key
[**disable_kms_key_rotation**](KmsKeyApi.md#disable_kms_key_rotation) | **POST** /kms-key/{id}/disable-key-rotation | Disable Key Rotation
[**enable_kms_key**](KmsKeyApi.md#enable_kms_key) | **POST** /kms-key/{id}/enable | Enable KMS Key
[**enable_kms_key_rotation**](KmsKeyApi.md#enable_kms_key_rotation) | **POST** /kms-key/{id}/enable-key-rotation | Enable Key Rotation
[**get_kms_key**](KmsKeyApi.md#get_kms_key) | **GET** /kms-key/{id} | Get KMS Key
[**list_kms_key_rotations**](KmsKeyApi.md#list_kms_key_rotations) | **GET** /kms-key/{id}/list-key-rotations | List KMS Key Rotations
[**list_kms_keys**](KmsKeyApi.md#list_kms_keys) | **GET** /kms-key | List KMS Keys
[**replicate_kms_key**](KmsKeyApi.md#replicate_kms_key) | **POST** /kms-key/{id}/replicate | Replicate KMS Key
[**rotate_kms_key**](KmsKeyApi.md#rotate_kms_key) | **POST** /kms-key/{id}/rotate | Rotate Key
[**schedule_kms_key_deletion**](KmsKeyApi.md#schedule_kms_key_deletion) | **POST** /kms-key/{id}/schedule-deletion | Schedule KMS Key Deletion



## cancel_kms_key_deletion

> models::SuccessResponse cancel_kms_key_deletion(id)
Cancel KMS Key Deletion

Cancels the scheduled deletion of a KMS Key.

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


## create_kms_key

> models::CreateKmsKeyResponse create_kms_key(create_kms_key_request)
Create KMS Key

Create a customer-managed unique KMS Key in your organization. A KMS Key is a logical represention of a cryptographic key material. It also includes metadata such as a UUID, a name and its state.

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
Disable KMS Key

Disables a KMS Key by setting its state to \"disabled\". This prevents the use of the KMS key for cryptographic and key lifecycle operations.

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

> models::DisableKmsKeyRotationResponse disable_kms_key_rotation(id)
Disable Key Rotation

Disable the periodic rotation of a KMS Key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DisableKmsKeyRotationResponse**](disable-kms-key-rotation-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_kms_key

> models::SuccessResponse enable_kms_key(id)
Enable KMS Key

Enables a KMS Key by setting its stated to \"enabled\". It restores the ability to fully use the KMS key for cryptographic operations and key lifecycle operations.

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
Enable Key Rotation

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
Get KMS Key

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
List KMS Key Rotations

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
List KMS Keys

Lists all KMS Keys in your organization in a given zone.

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

> models::SuccessResponse replicate_kms_key(id, replicate_kms_key_request)
Replicate KMS Key

Replicate a KMS key to a target zone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**replicate_kms_key_request** | [**ReplicateKmsKeyRequest**](ReplicateKmsKeyRequest.md) |  | [required] |

### Return type

[**models::SuccessResponse**](success-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_kms_key

> models::RotateKmsKeyResponse rotate_kms_key(id)
Rotate Key

Performs an immediate rotation of the key material for a symmetric key.

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

> models::ScheduleKmsKeyDeletionResponse schedule_kms_key_deletion(id, schedule_kms_key_deletion_request)
Schedule KMS Key Deletion

Schedules a KMS key for deletion after a delay. You can specify a delay of 7-30 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**schedule_kms_key_deletion_request** | [**ScheduleKmsKeyDeletionRequest**](ScheduleKmsKeyDeletionRequest.md) |  | [required] |

### Return type

[**models::ScheduleKmsKeyDeletionResponse**](schedule-kms-key-deletion-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

