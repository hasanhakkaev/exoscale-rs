# SecurityGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Security Group ID | [optional][readonly]
**name** | Option<**String**> | Security Group name | [optional]
**description** | Option<**String**> | Security Group description | [optional]
**external_sources** | Option<**HashSet<String>**> | Security Group external sources | [optional]
**rules** | Option<[**HashSet<models::SecurityGroupRule>**](SecurityGroupRule.md)> | Security Group rules | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


