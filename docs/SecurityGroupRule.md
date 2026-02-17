# SecurityGroupRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Security Group rule description | [optional]
**start_port** | Option<**u64**> | Start port of the range | [optional]
**protocol** | Option<**Protocol**> | Network protocol (enum: tcp, esp, icmp, udp, gre, ah, ipip, icmpv6) | [optional]
**icmp** | Option<[**models::SecurityGroupRuleIcmp**](SecurityGroupRuleIcmp.md)> |  | [optional]
**end_port** | Option<**u64**> | End port of the range | [optional]
**security_group** | Option<[**models::SecurityGroupResource**](SecurityGroupResource.md)> |  | [optional]
**id** | Option<**uuid::Uuid**> | Security Group rule ID | [optional][readonly]
**network** | Option<**String**> | CIDR-formatted network allowed | [optional]
**flow_direction** | Option<**FlowDirection**> | Network flow direction to match (enum: ingress, egress) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


