# LoadBalancerService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Load Balancer Service description | [optional]
**protocol** | Option<**Protocol**> | Network traffic protocol (enum: tcp, udp) | [optional]
**name** | Option<**String**> | Load Balancer Service name | [optional]
**state** | Option<**State**> | Load Balancer Service state (enum: creating, deleting, running, updating, error) | [optional][readonly]
**target_port** | Option<**u64**> | Port on which the network traffic will be forwarded to on the receiving instance | [optional]
**port** | Option<**u64**> | Port exposed on the Load Balancer's public IP | [optional]
**instance_pool** | Option<[**models::InstancePool**](InstancePool.md)> |  | [optional]
**strategy** | Option<**Strategy**> | Load balancing strategy (enum: round-robin, maglev-hash, source-hash) | [optional]
**healthcheck** | Option<[**models::LoadBalancerServiceHealthcheck**](LoadBalancerServiceHealthcheck.md)> |  | [optional]
**id** | Option<**uuid::Uuid**> | Load Balancer Service ID | [optional][readonly]
**healthcheck_status** | Option<[**Vec<models::LoadBalancerServerStatus>**](LoadBalancerServerStatus.md)> | Healthcheck status per backend server | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


