# JsonSchemaOpensearch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**thread_pool_search_throttled_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_analyze_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_get_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_get_queue_size** | Option<**i32**> | Size for the thread pool queue. See documentation for exact details. | [optional]
**indices_memory_max_index_buffer_size** | Option<**i32**> | Absolute value. Default is unbound. Doesn't work without indices.memory.index_buffer_size. Maximum amount of heap used for query cache, an absolute indices.memory.index_buffer_size maximum hard limit. | [optional]
**indices_recovery_max_concurrent_file_chunks** | Option<**i32**> | Number of file chunks sent in parallel for each recovery. Defaults to 2. | [optional]
**indices_queries_cache_size** | Option<**i32**> | Percentage value. Default is 10%. Maximum amount of heap used for query cache. This is an expert setting. Too low value will decrease query performance and increase performance for other operations; too high value will cause issues with other OpenSearch functionality. | [optional]
**search_backpressure** | Option<[**models::SearchBackpressureSettings**](Search_Backpressure_Settings.md)> |  | [optional]
**shard_indexing_pressure** | Option<[**models::ShardIndexingBackPressureSettings**](Shard_indexing_back_pressure_settings.md)> |  | [optional]
**knn_memory_circuit_breaker_enabled** | Option<**bool**> | Enable or disable KNN memory circuit breaker. Defaults to true. | [optional]
**thread_pool_search_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**indices_memory_min_index_buffer_size** | Option<**i32**> | Absolute value. Default is 48mb. Doesn't work without indices.memory.index_buffer_size. Minimum amount of heap used for query cache, an absolute indices.memory.index_buffer_size minimal hard limit. | [optional]
**indices_recovery_max_bytes_per_sec** | Option<**i32**> | Limits total inbound and outbound recovery traffic for each node. Applies to both peer recoveries as well as snapshot recoveries (i.e., restores from a snapshot). Defaults to 40mb | [optional]
**http_max_initial_line_length** | Option<**i32**> | The max length of an HTTP URL, in bytes | [optional]
**enable_security_audit** | Option<**bool**> |  | [optional][default to false]
**thread_pool_write_queue_size** | Option<**i32**> | Size for the thread pool queue. See documentation for exact details. | [optional]
**script_max_compilations_rate** | Option<**String**> | Script compilation circuit breaker limits the number of inline script compilations within a period of time. Default is use-context | [optional]
**search_max_buckets** | Option<**i32**> | Maximum number of aggregation buckets allowed in a single response. OpenSearch default value is used when this is not defined. | [optional]
**reindex_remote_whitelist** | Option<**Vec<String>**> | Whitelisted addresses for reindexing. Changing this value will cause all OpenSearch instances to restart. | [optional]
**override_main_response_version** | Option<**bool**> | Compatibility mode sets OpenSearch to report its version as 7.10 so clients continue to work. Default is false | [optional]
**http_max_header_size** | Option<**i32**> | The max size of allowed headers, in bytes | [optional]
**email_sender** | Option<[**models::OpensearchEmailSenderSettings**](Opensearch_Email_Sender_Settings.md)> |  | [optional]
**indices_fielddata_cache_size** | Option<**i32**> | Relative amount. Maximum amount of heap memory used for field data cache. This is an expert setting; decreasing the value too much will increase overhead of loading field data; too much memory used for field data cache will decrease amount of heap available for other operations. | [optional]
**action_destructive_requires_name** | Option<**bool**> |  | [optional]
**plugins_alerting_filter_by_backend_roles** | Option<**bool**> | Enable or disable filtering of alerting by backend roles. Requires Security plugin. Defaults to false | [optional]
**indices_memory_index_buffer_size** | Option<**i32**> | Percentage value. Default is 10%. Total amount of heap used for indexing buffer, before writing segments to disk. This is an expert setting. Too low value will slow down indexing; too high value will increase indexing performance but causes performance issues for query performance. | [optional]
**thread_pool_force_merge_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**auth_failure_listeners** | Option<[**models::OpensearchSecurityPluginSettings**](Opensearch_Security_Plugin_Settings.md)> |  | [optional]
**ism_history** | Option<[**models::OpensearchIsmHistorySettings**](Opensearch_ISM_History_Settings.md)> |  | [optional]
**cluster_routing_allocation_node_concurrent_recoveries** | Option<**i32**> | How many concurrent incoming/outgoing shard recoveries (normally replicas) are allowed to happen on a node. Defaults to 2. | [optional]
**thread_pool_analyze_queue_size** | Option<**i32**> | Size for the thread pool queue. See documentation for exact details. | [optional]
**action_auto_create_index_enabled** | Option<**bool**> | Explicitly allow or block automatic creation of indices. Defaults to true | [optional]
**http_max_content_length** | Option<**i32**> | Maximum content length for HTTP requests to the OpenSearch HTTP API, in bytes. | [optional]
**thread_pool_write_size** | Option<**i32**> | Size for the thread pool. See documentation for exact details. Do note this may have maximum value depending on CPU count - value is automatically lowered if set to higher than maximum value. | [optional]
**thread_pool_search_queue_size** | Option<**i32**> | Size for the thread pool queue. See documentation for exact details. | [optional]
**knn_memory_circuit_breaker_limit** | Option<**i32**> | Maximum amount of memory that can be used for KNN index. Defaults to 50% of the JVM heap size. | [optional]
**indices_query_bool_max_clause_count** | Option<**i32**> | Maximum number of clauses Lucene BooleanQuery can have. The default value (1024) is relatively high, and increasing it may cause performance issues. Investigate other approaches first before increasing this value. | [optional]
**thread_pool_search_throttled_queue_size** | Option<**i32**> | Size for the thread pool queue. See documentation for exact details. | [optional]
**cluster_max_shards_per_node** | Option<**i32**> | Controls the number of shards allowed in the cluster per data node | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


