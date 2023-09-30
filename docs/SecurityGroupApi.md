# \SecurityGroupApi

All URIs are relative to *https://api-ch-gva-2.exoscale.com/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_external_source_to_security_group**](SecurityGroupApi.md#add_external_source_to_security_group) | **PUT** /security-group/{id}:add-source | Add an external source as a member of a Security Group
[**add_rule_to_security_group**](SecurityGroupApi.md#add_rule_to_security_group) | **POST** /security-group/{id}/rules | Create a Security Group rule
[**attach_instance_to_security_group**](SecurityGroupApi.md#attach_instance_to_security_group) | **PUT** /security-group/{id}:attach | Attach a Compute instance to a Security Group
[**create_security_group**](SecurityGroupApi.md#create_security_group) | **POST** /security-group | Create a Security Group
[**delete_rule_from_security_group**](SecurityGroupApi.md#delete_rule_from_security_group) | **DELETE** /security-group/{id}/rules/{rule_id} | Delete a Security Group rule
[**delete_security_group**](SecurityGroupApi.md#delete_security_group) | **DELETE** /security-group/{id} | Delete a Security Group
[**detach_instance_from_security_group**](SecurityGroupApi.md#detach_instance_from_security_group) | **PUT** /security-group/{id}:detach | Detach a Compute instance from a Security Group
[**get_security_group**](SecurityGroupApi.md#get_security_group) | **GET** /security-group/{id} | Retrieve Security Group details
[**list_security_groups**](SecurityGroupApi.md#list_security_groups) | **GET** /security-group | List Security Groups.
[**remove_external_source_from_security_group**](SecurityGroupApi.md#remove_external_source_from_security_group) | **PUT** /security-group/{id}:remove-source | Remove an external source from a Security Group



## add_external_source_to_security_group

> crate::models::Operation add_external_source_to_security_group(id, add_external_source_to_security_group_request)
Add an external source as a member of a Security Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**add_external_source_to_security_group_request** | [**AddExternalSourceToSecurityGroupRequest**](AddExternalSourceToSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_rule_to_security_group

> crate::models::Operation add_rule_to_security_group(id, add_rule_to_security_group_request)
Create a Security Group rule



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**add_rule_to_security_group_request** | [**AddRuleToSecurityGroupRequest**](AddRuleToSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## attach_instance_to_security_group

> crate::models::Operation attach_instance_to_security_group(id, detach_instance_from_private_network_request)
Attach a Compute instance to a Security Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**detach_instance_from_private_network_request** | [**DetachInstanceFromPrivateNetworkRequest**](DetachInstanceFromPrivateNetworkRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_security_group

> crate::models::Operation create_security_group(create_security_group_request)
Create a Security Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_security_group_request** | [**CreateSecurityGroupRequest**](CreateSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rule_from_security_group

> crate::models::Operation delete_rule_from_security_group(id, rule_id)
Delete a Security Group rule



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**rule_id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group

> crate::models::Operation delete_security_group(id)
Delete a Security Group



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


## detach_instance_from_security_group

> crate::models::Operation detach_instance_from_security_group(id, detach_instance_from_private_network_request)
Detach a Compute instance from a Security Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**detach_instance_from_private_network_request** | [**DetachInstanceFromPrivateNetworkRequest**](DetachInstanceFromPrivateNetworkRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_security_group

> crate::models::SecurityGroup get_security_group(id)
Retrieve Security Group details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::SecurityGroup**](security-group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_security_groups

> crate::models::ListSecurityGroups200Response list_security_groups(visibility)
List Security Groups.

Lists security groups. When visibility is set to public, lists public security groups. Public security groups are objects maintained by Exoscale which contain source addresses for relevant services hosted by Exoscale. They can be used a source in ingress rules and as a destination in egress rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**visibility** | Option<**String**> |  |  |

### Return type

[**crate::models::ListSecurityGroups200Response**](list_security_groups_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_external_source_from_security_group

> crate::models::Operation remove_external_source_from_security_group(id, remove_external_source_from_security_group_request)
Remove an external source from a Security Group



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**remove_external_source_from_security_group_request** | [**RemoveExternalSourceFromSecurityGroupRequest**](RemoveExternalSourceFromSecurityGroupRequest.md) |  | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

