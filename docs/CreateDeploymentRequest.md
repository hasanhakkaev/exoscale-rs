# CreateDeploymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_count** | **u64** | Number of GPUs (1-8) | 
**inference_engine_version** | Option<[**models::InferenceEngineVersion**](InferenceEngineVersion.md)> |  | [optional]
**name** | Option<**String**> | Deployment name | [optional]
**gpu_type** | **String** | GPU type family (e.g., gpua5000, gpu3080ti) | 
**replicas** | **u64** | Number of replicas (>=1) | 
**inference_engine_parameters** | Option<**Vec<String>**> | Optional extra inference engine server CLI args | [optional]
**model** | Option<[**models::ModelRef**](ModelRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


