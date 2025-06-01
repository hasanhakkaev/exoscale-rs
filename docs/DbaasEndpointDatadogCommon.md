# DbaasEndpointDatadogCommon

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**datadog_tags** | Option<[**Vec<models::DbaasDatadogTag>**](dbaas-datadog-tag.md)> | Custom tags provided by user | [optional]
**disable_consumer_stats** | Option<**bool**> | Disable kafka consumer group metrics. Applies only when attached to kafka services. | [optional]
**kafka_consumer_check_instances** | Option<**u64**> | Number of separate instances to fetch kafka consumer statistics with. Applies only when attached to kafka services. | [optional]
**kafka_consumer_stats_timeout** | Option<**u64**> | Number of seconds that datadog will wait to get consumer statistics from brokers. Applies only when attached to kafka services. | [optional]
**max_partition_contexts** | Option<**u64**> | Maximum number of partition contexts to send. Applies only when attached to kafka services. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


