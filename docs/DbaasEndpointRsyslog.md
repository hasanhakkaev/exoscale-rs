# DbaasEndpointRsyslog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server** | **String** | Rsyslog server IP address or hostname | 
**port** | **i64** | Rsyslog server port | 
**tls** | **bool** | Require TLS | 
**format** | [**models::EnumRsyslogFormat**](enum-rsyslog-format.md) |  | 
**logline** | Option<**String**> | Custom syslog message format | [optional]
**sd** | Option<**String**> | Structured data block for log message | [optional]
**max_message_size** | Option<**i64**> | Rsyslog max message size | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


