# ThanosQueryUserConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_period_default_evaluation_interval** | Option<**String**> | Set the default evaluation interval for subqueries. | [optional][default to 1m]
**query_period_lookback_delta** | Option<**String**> | The maximum lookback duration for retrieving metrics during expression evaluations in PromQL. PromQL always evaluates the query for a certain timestamp, and it looks back for the given amount of time to get the latest sample. If it exceeds the maximum lookback delta, it assumes the series is stale and returns none (a gap). The lookback delta should be set to at least 2 times the slowest scrape interval. If unset, it will use the promql default of 5m. | [optional][default to 5m]
**query_period_metadata_period_default_time_range** | Option<**String**> | The default metadata time range duration for retrieving labels through Labels and Series API when the range parameters are not specified. The zero value means the range covers the time since the beginning. | [optional][default to 0s]
**query_period_timeout** | Option<**String**> | Maximum time to process a query by the query node. | [optional][default to 2m]
**store_period_limits_period_request_samples** | Option<**u32**> | The maximum samples allowed for a single Series request. The Series call fails if this limit is exceeded. Set to 0 for no limit. NOTE: For efficiency, the limit is internally implemented as 'chunks limit' considering each chunk contains a maximum of 120 samples. The default value is 100 * store.limits.request-series. | [optional][default to 0]
**store_period_limits_period_request_series** | Option<**u32**> | The maximum series allowed for a single Series request. The Series call fails if this limit is exceeded. Set to 0 for no limit. The default value is 1000 * cpu_count. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


