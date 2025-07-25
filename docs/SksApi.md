# \SksApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sks_cluster**](SksApi.md#create_sks_cluster) | **POST** /sks-cluster | Create an SKS cluster
[**create_sks_nodepool**](SksApi.md#create_sks_nodepool) | **POST** /sks-cluster/{id}/nodepool | Create a new SKS Nodepool
[**delete_sks_cluster**](SksApi.md#delete_sks_cluster) | **DELETE** /sks-cluster/{id} | Delete an SKS cluster
[**delete_sks_nodepool**](SksApi.md#delete_sks_nodepool) | **DELETE** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Delete an SKS Nodepool
[**evict_sks_nodepool_members**](SksApi.md#evict_sks_nodepool_members) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id}:evict | Evict Nodepool members
[**generate_sks_cluster_kubeconfig**](SksApi.md#generate_sks_cluster_kubeconfig) | **POST** /sks-cluster-kubeconfig/{id} | Generate a new Kubeconfig file for a SKS cluster
[**get_sks_cluster**](SksApi.md#get_sks_cluster) | **GET** /sks-cluster/{id} | Retrieve SKS cluster details
[**get_sks_cluster_authority_cert**](SksApi.md#get_sks_cluster_authority_cert) | **GET** /sks-cluster/{id}/authority/{authority}/cert | Get the certificate for a SKS cluster authority
[**get_sks_cluster_inspection**](SksApi.md#get_sks_cluster_inspection) | **GET** /sks-cluster/{id}/inspection | Get the latest inspection result
[**get_sks_nodepool**](SksApi.md#get_sks_nodepool) | **GET** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Retrieve SKS Nodepool details
[**list_sks_cluster_deprecated_resources**](SksApi.md#list_sks_cluster_deprecated_resources) | **GET** /sks-cluster-deprecated-resources/{id} | Resources that are scheduled to be removed in future kubernetes releases
[**list_sks_cluster_versions**](SksApi.md#list_sks_cluster_versions) | **GET** /sks-cluster-version | List available versions for SKS clusters
[**list_sks_clusters**](SksApi.md#list_sks_clusters) | **GET** /sks-cluster | List SKS clusters
[**reset_sks_cluster_field**](SksApi.md#reset_sks_cluster_field) | **DELETE** /sks-cluster/{id}/{field} | Reset an SKS cluster field to its default value
[**reset_sks_nodepool_field**](SksApi.md#reset_sks_nodepool_field) | **DELETE** /sks-cluster/{id}/nodepool/{sks_nodepool_id}/{field} | Reset an SKS Nodepool field to its default value
[**rotate_sks_ccm_credentials**](SksApi.md#rotate_sks_ccm_credentials) | **PUT** /sks-cluster/{id}/rotate-ccm-credentials | Rotate Exoscale CCM credentials
[**rotate_sks_csi_credentials**](SksApi.md#rotate_sks_csi_credentials) | **PUT** /sks-cluster/{id}/rotate-csi-credentials | Rotate Exoscale CSI credentials
[**rotate_sks_operators_ca**](SksApi.md#rotate_sks_operators_ca) | **PUT** /sks-cluster/{id}/rotate-operators-ca | Rotate operators certificate authority
[**scale_sks_nodepool**](SksApi.md#scale_sks_nodepool) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id}:scale | Scale a SKS Nodepool
[**update_sks_cluster**](SksApi.md#update_sks_cluster) | **PUT** /sks-cluster/{id} | Update an SKS cluster
[**update_sks_nodepool**](SksApi.md#update_sks_nodepool) | **PUT** /sks-cluster/{id}/nodepool/{sks_nodepool_id} | Update an SKS Nodepool
[**upgrade_sks_cluster**](SksApi.md#upgrade_sks_cluster) | **PUT** /sks-cluster/{id}/upgrade | Upgrade an SKS cluster
[**upgrade_sks_cluster_service_level**](SksApi.md#upgrade_sks_cluster_service_level) | **PUT** /sks-cluster/{id}/upgrade-service-level | Upgrade a SKS cluster to pro



## create_sks_cluster

> models::Operation create_sks_cluster(create_sks_cluster_request)
Create an SKS cluster



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sks_cluster_request** | [**CreateSksClusterRequest**](CreateSksClusterRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sks_nodepool

> models::Operation create_sks_nodepool(id, create_sks_nodepool_request)
Create a new SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**create_sks_nodepool_request** | [**CreateSksNodepoolRequest**](CreateSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sks_cluster

> models::Operation delete_sks_cluster(id)
Delete an SKS cluster



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


## delete_sks_nodepool

> models::Operation delete_sks_nodepool(id, sks_nodepool_id)
Delete an SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evict_sks_nodepool_members

> models::Operation evict_sks_nodepool_members(id, sks_nodepool_id, evict_sks_nodepool_members_request)
Evict Nodepool members

This operation evicts the specified Compute instances member from the Nodepool, shrinking it to `&lt;current nodepool size&gt; - &lt;# evicted members&gt;`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**evict_sks_nodepool_members_request** | [**EvictSksNodepoolMembersRequest**](EvictSksNodepoolMembersRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_sks_cluster_kubeconfig

> models::GenerateSksClusterKubeconfig200Response generate_sks_cluster_kubeconfig(id, sks_kubeconfig_request)
Generate a new Kubeconfig file for a SKS cluster

This operation returns a Kubeconfig file encoded in base64.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_kubeconfig_request** | [**SksKubeconfigRequest**](SksKubeconfigRequest.md) |  | [required] |

### Return type

[**models::GenerateSksClusterKubeconfig200Response**](generate_sks_cluster_kubeconfig_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sks_cluster

> models::SksCluster get_sks_cluster(id)
Retrieve SKS cluster details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SksCluster**](sks-cluster.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sks_cluster_authority_cert

> models::GetSksClusterAuthorityCert200Response get_sks_cluster_authority_cert(id, authority)
Get the certificate for a SKS cluster authority

This operation returns the certificate for the given SKS cluster authority encoded in base64.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**authority** | **String** |  | [required] |

### Return type

[**models::GetSksClusterAuthorityCert200Response**](get_sks_cluster_authority_cert_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sks_cluster_inspection

> std::collections::HashMap<String, serde_json::Value> get_sks_cluster_inspection(id)
Get the latest inspection result

Helps troubleshoot common problems when deploying a kubernetes cluster. Inspections run every couple of minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sks_nodepool

> models::SksNodepool get_sks_nodepool(id, sks_nodepool_id)
Retrieve SKS Nodepool details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SksNodepool**](sks-nodepool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sks_cluster_deprecated_resources

> Vec<models::SksClusterDeprecatedResource> list_sks_cluster_deprecated_resources(id)
Resources that are scheduled to be removed in future kubernetes releases

This operation returns the deprecated resources for a given cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**Vec<models::SksClusterDeprecatedResource>**](sks-cluster-deprecated-resource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sks_cluster_versions

> models::ListSksClusterVersions200Response list_sks_cluster_versions(include_deprecated)
List available versions for SKS clusters



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_deprecated** | Option<**String**> |  |  |

### Return type

[**models::ListSksClusterVersions200Response**](list_sks_cluster_versions_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sks_clusters

> models::ListSksClusters200Response list_sks_clusters()
List SKS clusters



### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSksClusters200Response**](list_sks_clusters_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_sks_cluster_field

> models::Operation reset_sks_cluster_field(id, field)
Reset an SKS cluster field to its default value



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_sks_nodepool_field

> models::Operation reset_sks_nodepool_field(id, sks_nodepool_id, field)
Reset an SKS Nodepool field to its default value



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**field** | **String** |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_sks_ccm_credentials

> models::Operation rotate_sks_ccm_credentials(id)
Rotate Exoscale CCM credentials



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


## rotate_sks_csi_credentials

> models::Operation rotate_sks_csi_credentials(id)
Rotate Exoscale CSI credentials



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


## rotate_sks_operators_ca

> models::Operation rotate_sks_operators_ca(id)
Rotate operators certificate authority



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


## scale_sks_nodepool

> models::Operation scale_sks_nodepool(id, sks_nodepool_id, scale_sks_nodepool_request)
Scale a SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**scale_sks_nodepool_request** | [**ScaleSksNodepoolRequest**](ScaleSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sks_cluster

> models::Operation update_sks_cluster(id, update_sks_cluster_request)
Update an SKS cluster



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_sks_cluster_request** | [**UpdateSksClusterRequest**](UpdateSksClusterRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sks_nodepool

> models::Operation update_sks_nodepool(id, sks_nodepool_id, update_sks_nodepool_request)
Update an SKS Nodepool



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**sks_nodepool_id** | **uuid::Uuid** |  | [required] |
**update_sks_nodepool_request** | [**UpdateSksNodepoolRequest**](UpdateSksNodepoolRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_sks_cluster

> models::Operation upgrade_sks_cluster(id, upgrade_sks_cluster_request)
Upgrade an SKS cluster



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**upgrade_sks_cluster_request** | [**UpgradeSksClusterRequest**](UpgradeSksClusterRequest.md) |  | [required] |

### Return type

[**models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_sks_cluster_service_level

> models::Operation upgrade_sks_cluster_service_level(id)
Upgrade a SKS cluster to pro



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

