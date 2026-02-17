# AddRuleToSecurityGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_direction** | **FlowDirection** | Network flow direction to match (enum: ingress, egress) | 
**description** | Option<**String**> | Security Group rule description | [optional]
**network** | Option<**String**> | CIDR-formatted network allowed | [optional]
**security_group** | Option<[**models::SecurityGroupResource**](SecurityGroupResource.md)> |  | [optional]
**protocol** | **Protocol** | Network protocol (enum: tcp, esp, icmp, udp, gre, ah, ipip, icmpv6) | 
**icmp** | Option<[**models::AddRuleToSecurityGroupRequestIcmp**](AddRuleToSecurityGroupRequestIcmp.md)> |  | [optional]
**start_port** | Option<**u64**> | Start port of the range | [optional]
**end_port** | Option<**u64**> | End port of the range | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


