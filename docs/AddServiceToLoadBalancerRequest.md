# AddServiceToLoadBalancerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Load Balancer Service name | 
**description** | Option<**String**> | Load Balancer Service description | [optional]
**instance_pool** | [**crate::models::InstancePool**](instance-pool.md) |  | 
**protocol** | **String** | Network traffic protocol | 
**strategy** | **String** | Load balancing strategy | 
**port** | **i64** | Port exposed on the Load Balancer's public IP | 
**target_port** | **i64** | Port on which the network traffic will be forwarded to on the receiving instance | 
**healthcheck** | [**crate::models::LoadBalancerServiceHealthcheck**](load-balancer-service-healthcheck.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


