# JsonSchemaKafkaRest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**producer_compression_type** | Option<**String**> | Specify the default compression type for producers. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'none' which is the default and equivalent to no compression. | [optional]
**name_strategy_validation** | Option<**bool**> | If true, validate that given schema is registered under expected subject name by the used name strategy when producing messages. | [optional][default to true]
**name_strategy** | Option<**String**> | Name strategy to use when selecting subject for storing schemas | [optional][default to TopicName]
**consumer_enable_auto_commit** | Option<**bool**> | If true the consumer's offset will be periodically committed to Kafka in the background | [optional][default to true]
**producer_acks** | Option<**String**> | The number of acknowledgments the producer requires the leader to have received before considering a request complete. If set to 'all' or '-1', the leader will wait for the full set of in-sync replicas to acknowledge the record. | [optional][default to Variant12]
**consumer_request_max_bytes** | Option<**i32**> | Maximum number of bytes in unencoded message keys and values by a single request | [optional][default to 67108864]
**producer_max_request_size** | Option<**i32**> | The maximum size of a request in bytes. Note that Kafka broker can also cap the record batch size. | [optional][default to 1048576]
**simpleconsumer_pool_size_max** | Option<**i32**> | Maximum number of SimpleConsumers that can be instantiated per broker | [optional][default to 25]
**producer_linger_ms** | Option<**i32**> | Wait for up to the given delay to allow batching records together | [optional][default to 0]
**consumer_request_timeout_ms** | Option<**i32**> | The maximum total time to wait for messages for a request if the maximum number of messages has not yet been reached | [optional][default to Variant1000]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


