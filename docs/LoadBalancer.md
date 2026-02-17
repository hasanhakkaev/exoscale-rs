# LoadBalancer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Load Balancer ID | [optional][readonly]
**description** | Option<**String**> | Load Balancer description | [optional]
**name** | Option<**String**> | Load Balancer name | [optional]
**state** | Option<**State**> | Load Balancer state (enum: creating, migrated, deleting, running, migrating, error) | [optional][readonly]
**created_at** | Option<**String**> | Load Balancer creation date | [optional][readonly]
**ip** | Option<**String**> | Load Balancer public IP | [optional][readonly]
**services** | Option<[**Vec<models::LoadBalancerService>**](LoadBalancerService.md)> | Load Balancer Services | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


