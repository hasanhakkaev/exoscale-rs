# DbaasEndpointRsyslogInputUpdateSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | Option<[**models::EnumRsyslogFormat**](enum-rsyslog-format.md)> |  | [optional]
**key** | Option<**String**> | PEM encoded client key | [optional]
**logline** | Option<**String**> | Custom syslog message format | [optional]
**server** | Option<**String**> | Rsyslog server IP address or hostname | [optional]
**ca** | Option<**String**> | PEM encoded CA certificate | [optional]
**cert** | Option<**String**> | PEM encoded client certificate | [optional]
**tls** | Option<**bool**> | Require TLS | [optional]
**port** | Option<**u64**> | Rsyslog server port | [optional]
**sd** | Option<**String**> | Structured data block for log message | [optional]
**max_message_size** | Option<**u64**> | Rsyslog max message size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


