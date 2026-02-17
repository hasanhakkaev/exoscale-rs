# AccessKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | IAM Access Key name | [optional]
**key** | Option<**String**> | IAM Access Key | [optional]
**secret** | Option<**String**> | IAM Access Key Secret | [optional][readonly]
**r#type** | Option<**Type**> | IAM Access Key type (enum: restricted, unrestricted) | [optional][readonly]
**version** | Option<**Version**> | IAM Access Key version (enum: v2, v1) | [optional]
**tags** | Option<**Vec<String>**> | IAM Access Key tags | [optional]
**operations** | Option<**Vec<String>**> | IAM Access Key operations | [optional]
**resources** | Option<[**Vec<models::AccessKeyResource>**](AccessKeyResource.md)> | IAM Access Key Resources | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


