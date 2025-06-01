# SearchTaskSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_heap_percent_threshold** | Option<**f64**> | The heap usage threshold (as a percentage) required for the sum of heap usages of all search tasks before cancellation is applied. Default is 0.5 | [optional]
**elapsed_time_millis_threshold** | Option<**u32**> | The elapsed time threshold (in milliseconds) required for an individual parent task before it is considered for cancellation. Default is 45000 | [optional]
**cancellation_rate** | Option<**f64**> | The maximum number of search tasks to cancel per millisecond of elapsed time. Default is 0.003 | [optional]
**heap_variance** | Option<**f64**> | The heap usage variance required for an individual parent task before it is considered for cancellation. A task is considered for cancellation when taskHeapUsage is greater than or equal to heapUsageMovingAverage * variance. Default is 2.0 | [optional]
**heap_moving_average_window_size** | Option<**u32**> | The window size used to calculate the rolling average of the heap usage for the completed parent tasks. Default is 10 | [optional]
**cancellation_ratio** | Option<**f64**> | The maximum number of search tasks to cancel, as a percentage of successful search task completions. Default is 0.1 | [optional]
**heap_percent_threshold** | Option<**f64**> | The heap usage threshold (as a percentage) required for an individual parent task before it is considered for cancellation. Default is 0.2 | [optional]
**cpu_time_millis_threshold** | Option<**u32**> | The CPU usage threshold (in milliseconds) required for an individual parent task before it is considered for cancellation. Default is 30000 | [optional]
**cancellation_burst** | Option<**f64**> | The maximum number of search tasks to cancel in a single iteration of the observer thread. Default is 5.0 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


