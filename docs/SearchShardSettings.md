# SearchShardSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_heap_percent_threshold** | Option<**f64**> | The heap usage threshold (as a percentage) required for the sum of heap usages of all search shard tasks before cancellation is applied. Default is 0.5 | [optional]
**elapsed_time_millis_threshold** | Option<**u32**> | The elapsed time threshold (in milliseconds) required for a single search shard task before it is considered for cancellation. Default is 30000 | [optional]
**cancellation_rate** | Option<**f64**> | The maximum number of tasks to cancel per millisecond of elapsed time. Default is 0.003 | [optional]
**heap_variance** | Option<**f64**> | The minimum variance required for a single search shard task’s heap usage compared to the rolling average of previously completed tasks before it is considered for cancellation. Default is 2.0 | [optional]
**heap_moving_average_window_size** | Option<**u32**> | The number of previously completed search shard tasks to consider when calculating the rolling average of heap usage. Default is 100 | [optional]
**cancellation_ratio** | Option<**f64**> | The maximum number of tasks to cancel, as a percentage of successful task completions. Default is 0.1 | [optional]
**heap_percent_threshold** | Option<**f64**> | The heap usage threshold (as a percentage) required for a single search shard task before it is considered for cancellation. Default is 0.5 | [optional]
**cpu_time_millis_threshold** | Option<**u32**> | The CPU usage threshold (in milliseconds) required for a single search shard task before it is considered for cancellation. Default is 15000 | [optional]
**cancellation_burst** | Option<**f64**> | The maximum number of search tasks to cancel in a single iteration of the observer thread. Default is 10.0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


