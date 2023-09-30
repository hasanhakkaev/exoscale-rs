# SecurityGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Security Group ID | [optional][readonly]
**name** | Option<**String**> | Security Group name | [optional]
**description** | Option<**String**> | Security Group description | [optional]
**external_sources** | Option<**Vec<String>**> | Security Group external sources | [optional]
**rules** | Option<[**Vec<crate::models::SecurityGroupRule>**](security-group-rule.md)> | Security Group rules | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


