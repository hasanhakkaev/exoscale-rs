# GetDeploymentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_count** | **u64** | Number of GPUs | 
**updated_at** | **String** | Update time | [readonly]
**deployment_url** | **String** | Deployment inference endpoint URL | 
**service_level** | **String** | Service level | 
**inference_engine_version** | [**models::InferenceEngineVersion**](InferenceEngineVersion.md) |  | 
**name** | **String** | Deployment name | 
**state** | **State** | Deployment state (enum: ready, creating, preparing, error, deploying) | 
**gpu_type** | **String** | GPU type family | 
**id** | **uuid::Uuid** | Deployment ID | [readonly]
**replicas** | **u64** | Number of replicas (>=0) | 
**state_details** | **String** | Deployment state details | 
**created_at** | **String** | Creation time | [readonly]
**inference_engine_parameters** | **Vec<String>** | Optional extra inference engine server CLI args | 
**model** | Option<[**models::ModelRef**](ModelRef.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


