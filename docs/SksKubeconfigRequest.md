# SksKubeconfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ttl** | Option<**u64**> | Validity in seconds of the Kubeconfig user certificate (default: 30 days) | [optional]
**user** | **String** | User name in the generated Kubeconfig. The certificate present in the Kubeconfig will also have this name set for the CN field. | 
**groups** | **Vec<String>** | List of roles. The certificate present in the Kubeconfig will have these roles set in the Org field. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


