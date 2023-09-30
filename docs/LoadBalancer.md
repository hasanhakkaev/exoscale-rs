# LoadBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Load Balancer ID | [optional][readonly]
**description** | Option<**String**> | Load Balancer description | [optional]
**name** | Option<**String**> | Load Balancer name | [optional]
**state** | Option<**String**> | Load Balancer state | [optional][readonly]
**created_at** | Option<**String**> | Load Balancer creation date | [optional][readonly]
**ip** | Option<**String**> | Load Balancer public IP | [optional][readonly]
**services** | Option<[**Vec<crate::models::LoadBalancerService>**](load-balancer-service.md)> | Load Balancer Services | [optional]
**labels** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


