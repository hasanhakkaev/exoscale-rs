# GetDeploymentResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_count** | Option<**u64**> | Number of GPUs | [optional]
**updated_at** | Option<**String**> | Update time | [optional][readonly]
**deployment_url** | Option<**String**> | Deployment URL (nullable) | [optional]
**service_level** | Option<**String**> | Service level | [optional]
**inference_engine_version** | Option<[**models::InferenceEngineVersion**](InferenceEngineVersion.md)> |  | [optional]
**name** | Option<**String**> | Deployment name | [optional]
**state** | Option<**State**> | Deployment state (enum: ready, creating, error, deploying) | [optional]
**gpu_type** | Option<**String**> | GPU type family | [optional]
**id** | Option<**uuid::Uuid**> | Deployment ID | [optional][readonly]
**replicas** | Option<**u64**> | Number of replicas (>=0) | [optional]
**state_details** | Option<**String**> | Deployment state details | [optional]
**created_at** | Option<**String**> | Creation time | [optional][readonly]
**inference_engine_parameters** | Option<**Vec<String>**> | Optional extra inference engine server CLI args | [optional]
**model** | Option<[**models::ModelRef**](ModelRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


