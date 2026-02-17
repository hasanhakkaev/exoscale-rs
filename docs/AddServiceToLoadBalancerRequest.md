# AddServiceToLoadBalancerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Load Balancer Service name | 
**description** | Option<**String**> | Load Balancer Service description | [optional]
**instance_pool** | [**models::InstancePool**](InstancePool.md) |  | 
**protocol** | **Protocol** | Network traffic protocol (enum: tcp, udp) | 
**strategy** | **Strategy** | Load balancing strategy (enum: round-robin, maglev-hash, source-hash) | 
**port** | **u64** | Port exposed on the Load Balancer's public IP | 
**target_port** | **u64** | Port on which the network traffic will be forwarded to on the receiving instance | 
**healthcheck** | [**models::LoadBalancerServiceHealthcheck**](LoadBalancerServiceHealthcheck.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


