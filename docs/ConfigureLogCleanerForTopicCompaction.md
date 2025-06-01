# ConfigureLogCleanerForTopicCompaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**log_cleaner_delete_retention_ms** | Option<**u64**> | How long are delete records retained? | [optional]
**log_cleaner_max_compaction_lag_ms** | Option<**u64**> | The maximum amount of time message will remain uncompacted. Only applicable for logs that are being compacted | [optional]
**log_cleaner_min_cleanable_ratio** | Option<**f64**> | Controls log compactor frequency. Larger value means more frequent compactions but also more space wasted for logs. Consider setting log.cleaner.max.compaction.lag.ms to enforce compactions sooner, instead of setting a very high value for this option. | [optional]
**log_cleaner_min_compaction_lag_ms** | Option<**u64**> | The minimum time a message will remain uncompacted in the log. Only applicable for logs that are being compacted. | [optional]
**log_cleanup_policy** | Option<**String**> | The default cleanup policy for segments beyond the retention window | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


