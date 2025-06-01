# AddRuleToSecurityGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_direction** | **String** | Network flow direction to match | 
**description** | Option<**String**> | Security Group rule description | [optional]
**network** | Option<**String**> | CIDR-formatted network allowed | [optional]
**security_group** | Option<[**models::SecurityGroupResource**](security-group-resource.md)> |  | [optional]
**protocol** | **String** | Network protocol | 
**icmp** | Option<[**models::AddRuleToSecurityGroupRequestIcmp**](add_rule_to_security_group_request_icmp.md)> |  | [optional]
**start_port** | Option<**u64**> | Start port of the range | [optional]
**end_port** | Option<**u64**> | End port of the range | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


