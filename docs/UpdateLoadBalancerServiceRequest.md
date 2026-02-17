# UpdateLoadBalancerServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Load Balancer Service name | [optional]
**description** | Option<**String**> | Load Balancer Service description | [optional]
**protocol** | Option<**Protocol**> | Network traffic protocol (enum: tcp, udp) | [optional]
**strategy** | Option<**Strategy**> | Load balancing strategy (enum: round-robin, maglev-hash, source-hash) | [optional]
**port** | Option<**u64**> | Port exposed on the Load Balancer's public IP | [optional]
**target_port** | Option<**u64**> | Port on which the network traffic will be forwarded to on the receiving instance | [optional]
**healthcheck** | Option<[**models::LoadBalancerServiceHealthcheck**](LoadBalancerServiceHealthcheck.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


