# JsonSchemaPg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**track_activity_query_size** | Option<**u16**> | Specifies the number of bytes reserved to track the currently executing command for each active session. | [optional]
**timezone** | Option<**String**> | PostgreSQL service timezone | [optional]
**track_io_timing** | Option<**String**> | Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. | [optional]
**pg_stat_monitor_pgsm_enable_query_plan** | Option<**bool**> | Enables or disables query plan monitoring | [optional]
**max_files_per_process** | Option<**u16**> | PostgreSQL maximum number of files that can be open per process | [optional]
**pg_stat_monitor_pgsm_max_buckets** | Option<**u8**> | Sets the maximum number of buckets  | [optional]
**wal** | Option<[**models::WriteAheadLogWalSettings**](Write_ahead_log__WAL__settings.md)> |  | [optional]
**default_toast_compression** | Option<**String**> | Specifies the default TOAST compression method for values of compressible columns (the default is lz4). | [optional]
**deadlock_timeout** | Option<**u32**> | This is the amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition. | [optional]
**idle_in_transaction_session_timeout** | Option<**u32**> | Time out sessions with open transactions after this number of milliseconds | [optional]
**max_pred_locks_per_transaction** | Option<**u16**> | PostgreSQL maximum predicate locks per transaction | [optional]
**max_replication_slots** | Option<**u8**> | PostgreSQL maximum replication slots | [optional]
**autovacuum** | Option<[**models::AutovacuumSettings**](Autovacuum_settings.md)> |  | [optional]
**max_parallel_workers_per_gather** | Option<**u8**> | Sets the maximum number of workers that can be started by a single Gather or Gather Merge node | [optional]
**pg_partman_bgw_interval** | Option<**u32**> | Sets the time interval to run pg_partman's scheduled tasks | [optional]
**log_line_prefix** | Option<**String**> | Choose from one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze etc. | [optional]
**log_temp_files** | Option<**i32**> | Log statements for each temporary file created larger than this number of kilobytes, -1 disables | [optional]
**max_locks_per_transaction** | Option<**u16**> | PostgreSQL maximum locks per transaction | [optional]
**track_commit_timestamp** | Option<**String**> | Record commit time of transactions. | [optional]
**track_functions** | Option<**String**> | Enables tracking of function call counts and time used. | [optional]
**max_stack_depth** | Option<**u32**> | Maximum depth of the stack in bytes | [optional]
**max_parallel_workers** | Option<**u8**> | Sets the maximum number of workers that the system can support for parallel queries | [optional]
**pg_partman_bgw_role** | Option<**String**> | Controls which role to use for pg_partman's scheduled background tasks. | [optional]
**max_logical_replication_workers** | Option<**u8**> | PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers) | [optional]
**max_prepared_transactions** | Option<**u16**> | PostgreSQL maximum prepared transactions | [optional]
**max_worker_processes** | Option<**u8**> | Sets the maximum number of background processes that the system can support | [optional]
**pg_stat_statements_track** | Option<**String**> | Controls which statements are counted. Specify top to track top-level statements (those issued directly by clients), all to also track nested statements (such as statements invoked within functions), or none to disable statement statistics collection. The default value is top. | [optional]
**temp_file_limit** | Option<**i32**> | PostgreSQL temporary file limit in KiB, -1 for unlimited | [optional]
**log_error_verbosity** | Option<**String**> | Controls the amount of detail written in the server log for each message that is logged. | [optional]
**log_min_duration_statement** | Option<**i32**> | Log statements that take more than this number of milliseconds to run, -1 disables | [optional]
**max_standby_streaming_delay** | Option<**u32**> | Max standby streaming delay in milliseconds | [optional]
**jit** | Option<**bool**> | Controls system-wide use of Just-in-Time Compilation (JIT). | [optional]
**max_standby_archive_delay** | Option<**u32**> | Max standby archive delay in milliseconds | [optional]
**bg_writer** | Option<[**models::BackgroundBgWriterSettings**](Background__BG__writer_settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


