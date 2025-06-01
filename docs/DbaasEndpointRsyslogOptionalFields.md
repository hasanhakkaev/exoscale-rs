# DbaasEndpointRsyslogOptionalFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server** | Option<**String**> | Rsyslog server IP address or hostname | [optional]
**port** | Option<**u64**> | Rsyslog server port | [optional]
**tls** | Option<**bool**> | Require TLS | [optional]
**format** | Option<[**models::EnumRsyslogFormat**](enum-rsyslog-format.md)> |  | [optional]
**logline** | Option<**String**> | Custom syslog message format | [optional]
**sd** | Option<**String**> | Structured data block for log message | [optional]
**max_message_size** | Option<**u64**> | Rsyslog max message size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


