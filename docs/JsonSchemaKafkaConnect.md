# JsonSchemaKafkaConnect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**producer_buffer_memory** | Option<**i32**> | The total bytes of memory the producer can use to buffer records waiting to be sent to the broker (defaults to 33554432). | [optional]
**consumer_max_poll_interval_ms** | Option<**i32**> | The maximum delay in milliseconds between invocations of poll() when using consumer group management (defaults to 300000). | [optional]
**producer_compression_type** | Option<**String**> | Specify the default compression type for producers. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'none' which is the default and equivalent to no compression. | [optional]
**connector_client_config_override_policy** | Option<**String**> | Defines what client configurations can be overridden by the connector. Default is None | [optional]
**offset_flush_interval_ms** | Option<**i32**> | The interval at which to try committing offsets for tasks (defaults to 60000). | [optional]
**scheduled_rebalance_max_delay_ms** | Option<**i32**> | The maximum delay that is scheduled in order to wait for the return of one or more departed workers before rebalancing and reassigning their connectors and tasks to the group. During this period the connectors and tasks of the departed workers remain unassigned. Defaults to 5 minutes. | [optional]
**consumer_fetch_max_bytes** | Option<**i32**> | Records are fetched in batches by the consumer, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that the consumer can make progress. As such, this is not a absolute maximum. | [optional]
**consumer_max_partition_fetch_bytes** | Option<**i32**> | Records are fetched in batches by the consumer.If the first record batch in the first non-empty partition of the fetch is larger than this limit, the batch will still be returned to ensure that the consumer can make progress.  | [optional]
**offset_flush_timeout_ms** | Option<**i32**> | Maximum number of milliseconds to wait for records to flush and partition offset data to be committed to offset storage before cancelling the process and restoring the offset data to be committed in a future attempt (defaults to 5000). | [optional]
**consumer_auto_offset_reset** | Option<**String**> | What to do when there is no initial offset in Kafka or if the current offset does not exist any more on the server. Default is earliest | [optional]
**producer_max_request_size** | Option<**i32**> | This setting will limit the number of record batches the producer will send in a single request to avoid sending huge requests. | [optional]
**producer_batch_size** | Option<**i32**> | This setting gives the upper bound of the batch size to be sent. If there are fewer than this many bytes accumulated for this partition, the producer will 'linger' for the linger.ms time waiting for more records to show up. A batch size of zero will disable batching entirely (defaults to 16384). | [optional]
**session_timeout_ms** | Option<**i32**> | The timeout in milliseconds used to detect failures when using Kafka’s group management facilities (defaults to 10000). | [optional]
**producer_linger_ms** | Option<**i32**> | This setting gives the upper bound on the delay for batching: once there is batch.size worth of records for a partition it will be sent immediately regardless of this setting, however if there are fewer than this many bytes accumulated for this partition the producer will 'linger' for the specified time waiting for more records to show up. Defaults to 0. | [optional]
**consumer_isolation_level** | Option<**String**> | Transaction read isolation level. read_uncommitted is the default, but read_committed can be used if consume-exactly-once behavior is desired. | [optional]
**consumer_max_poll_records** | Option<**i32**> | The maximum number of records returned in a single call to poll() (defaults to 500). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


