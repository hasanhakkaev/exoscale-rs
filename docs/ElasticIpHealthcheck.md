# ElasticIpHealthcheck

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**strikes_ok** | Option<**i64**> | Number of attempts before considering the target healthy (default: 2) | [optional]
**tls_skip_verify** | Option<**bool**> | Skip TLS verification | [optional]
**tls_sni** | Option<**String**> | An optional domain or subdomain to check TLS against | [optional]
**strikes_fail** | Option<**i64**> | Number of attempts before considering the target unhealthy (default: 3) | [optional]
**mode** | **String** | Health check mode | 
**port** | **i64** | Health check port | 
**uri** | Option<**String**> | An endpoint to use for the health check, for example '/status' | [optional]
**interval** | Option<**i64**> | Interval between the checks in seconds (default: 10) | [optional]
**timeout** | Option<**i64**> | Health check timeout value in seconds (default: 2) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


