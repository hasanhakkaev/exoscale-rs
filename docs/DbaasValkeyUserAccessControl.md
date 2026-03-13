# DbaasValkeyUserAccessControl

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | Option<**Vec<String>**> | Use +@<category> to allow and -@<category> to disallow. Separate entries with a single space. Example: +@all -@dangerous. | [optional]
**channels** | Option<**Vec<String>**> | Patterns use standard glob syntax and must be separated by a single space. Example: ~* &events. | [optional]
**commands** | Option<**Vec<String>**> | Use +<command> to allow and -<command> to disallow. You can also use @<category>. Separate entries with a single space. Example: +@all -flushall. | [optional]
**keys** | Option<**Vec<String>**> | Patterns use standard glob syntax and must be separated by a single space. Example: cache:* session:*. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


