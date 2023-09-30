# UpdateLoadBalancerServiceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Load Balancer Service name | [optional]
**description** | Option<**String**> | Load Balancer Service description | [optional]
**protocol** | Option<**String**> | Network traffic protocol | [optional]
**strategy** | Option<**String**> | Load balancing strategy | [optional]
**port** | Option<**i64**> | Port exposed on the Load Balancer's public IP | [optional]
**target_port** | Option<**i64**> | Port on which the network traffic will be forwarded to on the receiving instance | [optional]
**healthcheck** | Option<[**crate::models::LoadBalancerServiceHealthcheck**](load-balancer-service-healthcheck.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


