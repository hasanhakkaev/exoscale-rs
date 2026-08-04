# ListDeploymentsResponseEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gpu_count** | Option<**u64**> | Number of GPUs | [optional]
**updated_at** | Option<**String**> | Update time | [optional][readonly]
**deployment_url** | **String** | Deployment inference endpoint URL | 
**service_level** | Option<**String**> | Service level | [optional]
**name** | **String** | Deployment name | 
**state** | **State** | Deployment state (enum: ready, creating, preparing, error, deploying, scaling, updating) | 
**gpu_type** | Option<**String**> | GPU type family | [optional]
**id** | Option<**uuid::Uuid**> | Deployment ID | [optional][readonly]
**replicas** | Option<**u64**> | Number of replicas (>=0) | [optional]
**created_at** | Option<**String**> | Creation time | [optional][readonly]
**visibility** | **Visibility** | Deployment visibility: private for your organization's deployments, public for Exoscale Managed Inference deployments. (enum: public, private) | 
**model** | [**models::ModelRef**](ModelRef.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


