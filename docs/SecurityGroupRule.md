# SecurityGroupRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Security Group rule description | [optional]
**start_port** | Option<**u64**> | Start port of the range | [optional]
**protocol** | Option<**String**> | Network protocol | [optional]
**icmp** | Option<[**models::SecurityGroupRuleIcmp**](security_group_rule_icmp.md)> |  | [optional]
**end_port** | Option<**u64**> | End port of the range | [optional]
**security_group** | Option<[**models::SecurityGroupResource**](security-group-resource.md)> |  | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Security Group rule ID | [optional][readonly]
**network** | Option<**String**> | CIDR-formatted network allowed | [optional]
**flow_direction** | Option<**String**> | Network flow direction to match | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


