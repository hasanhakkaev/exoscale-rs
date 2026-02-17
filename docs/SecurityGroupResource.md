# SecurityGroupResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Security Group ID | [optional][readonly]
**name** | Option<**String**> | Security Group name | [optional]
**visibility** | Option<**Visibility**> | Whether this points to a public security group. This is only valid when in the context of                    a rule addition which uses a public security group as a source or destination. (enum: private, public) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


