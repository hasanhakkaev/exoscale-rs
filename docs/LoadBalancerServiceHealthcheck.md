# LoadBalancerServiceHealthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**String**> | Healthcheck mode | [optional]
**interval** | Option<**i64**> | Healthcheck interval (default: 10). Must be greater than or equal to Timeout | [optional]
**uri** | Option<**String**> | An endpoint to use for the HTTP healthcheck, e.g. '/status' | [optional]
**port** | Option<**i64**> | Healthcheck port | [optional]
**timeout** | Option<**i64**> | Healthcheck timeout value (default: 2). Must be lower than or equal to Interval | [optional]
**retries** | Option<**i64**> | Number of retries before considering a Service failed | [optional]
**tls_sni** | Option<**String**> | SNI domain for HTTPS healthchecks | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


