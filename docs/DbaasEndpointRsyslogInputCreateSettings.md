# DbaasEndpointRsyslogInputCreateSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**format** | [**models::EnumRsyslogFormat**](enum-rsyslog-format.md) |  | 
**key** | Option<**String**> | PEM encoded client key | [optional]
**logline** | Option<**String**> | Custom syslog message format | [optional]
**server** | **String** | Rsyslog server IP address or hostname | 
**ca** | Option<**String**> | PEM encoded CA certificate | [optional]
**cert** | Option<**String**> | PEM encoded client certificate | [optional]
**tls** | **bool** | Require TLS | 
**port** | **i64** | Rsyslog server port | 
**sd** | Option<**String**> | Structured data block for log message | [optional]
**max_message_size** | Option<**i64**> | Rsyslog max message size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


