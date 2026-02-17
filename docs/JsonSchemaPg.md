# JsonSchemaPg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**track_activity_query_size** | Option<**u16**> | Specifies the number of bytes reserved to track the currently executing command for each active session. Changing this parameter causes a service restart. | [optional]
**timezone** | Option<**String**> | PostgreSQL service timezone | [optional]
**track_io_timing** | Option<**TrackIoTiming**> | Enables timing of database I/O calls. The default is `off`. When on, it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms. (enum: off, on) | [optional]
**pg_stat_monitor_pgsm_enable_query_plan** | Option<**bool**> | Enables or disables query plan monitoring. Changing this parameter causes a service restart. Only available for PostgreSQL 13+. | [optional]
**max_files_per_process** | Option<**u16**> | PostgreSQL maximum number of files that can be open per process. The default is `1000` (upstream default). Changing this parameter causes a service restart. | [optional]
**pg_stat_monitor_pgsm_max_buckets** | Option<**u8**> | Sets the maximum number of buckets. Changing this parameter causes a service restart. Only available for PostgreSQL 13+. | [optional]
**io_max_concurrency** | Option<**i16**> | EXPERIMENTAL: Controls the maximum number of I/O operations that one process can execute simultaneously. Version 18 and up only. Changing this parameter causes a service restart. | [optional][default to -1]
**wal** | Option<[**models::WriteAheadLogWalSettings**](WriteAheadLogWALSettings.md)> |  | [optional]
**default_toast_compression** | Option<**DefaultToastCompression**> | Specifies the default TOAST compression method for values of compressible columns. The default is `lz4`. Only available for PostgreSQL 14+. (enum: lz4, pglz) | [optional]
**deadlock_timeout** | Option<**u32**> | This is the amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition. The default is `1000` (upstream default). | [optional]
**idle_in_transaction_session_timeout** | Option<**u32**> | Time out sessions with open transactions after this number of milliseconds | [optional]
**max_pred_locks_per_transaction** | Option<**u16**> | PostgreSQL maximum predicate locks per transaction. The default is `64` (upstream default). Changing this parameter causes a service restart. | [optional]
**max_replication_slots** | Option<**u16**> | PostgreSQL maximum replication slots. The default is `20`. Changing this parameter causes a service restart. | [optional]
**max_sync_workers_per_subscription** | Option<**u8**> | Maximum number of synchronization workers per subscription. The default is `2`. | [optional]
**autovacuum** | Option<[**models::AutovacuumSettings**](AutovacuumSettings.md)> |  | [optional]
**max_parallel_workers_per_gather** | Option<**u8**> | Sets the maximum number of workers that can be started by a single Gather or Gather Merge node. The default is `2` (upstream default). | [optional]
**io_combine_limit** | Option<**u8**> | EXPERIMENTAL: Controls the largest I/O size in operations that combine I/O in 8kB units. Version 17 and up only. | [optional][default to 16]
**password_encryption** | Option<**PasswordEncryption**> | Chooses the algorithm for encrypting passwords. (enum: md5, scram-sha-256) | [optional]
**io_workers** | Option<**u8**> | EXPERIMENTAL: Number of IO worker processes, for io_method=worker. Version 18 and up only. Changing this parameter causes a service restart. | [optional][default to 3]
**pg_partman_bgw_interval** | Option<**u32**> | Sets the time interval in seconds to run pg_partman's scheduled tasks. The default is `3600`. | [optional]
**log_line_prefix** | Option<**LogLinePrefix**> | Choose from one of the available log formats. (enum: 'pid=%p,user=%u,db=%d,app=%a,client=%h ', 'pid=%p,user=%u,db=%d,app=%a,client=%h,txid=%x,qid=%Q ', '%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h ', '%m [%p] %q[user=%u,db=%d,app=%a] ') | [optional]
**log_temp_files** | Option<**i32**> | Log statements for each temporary file created larger than this number of kilobytes, -1 disables | [optional]
**max_locks_per_transaction** | Option<**u16**> | PostgreSQL maximum locks per transaction. Changing this parameter causes a service restart. | [optional]
**track_commit_timestamp** | Option<**TrackCommitTimestamp**> | Record commit time of transactions. Changing this parameter causes a service restart. (enum: off, on) | [optional]
**track_functions** | Option<**TrackFunctions**> | Enables tracking of function call counts and time used. (enum: all, pl, none) | [optional]
**io_max_combine_limit** | Option<**u8**> | EXPERIMENTAL: Controls the largest I/O size in operations that combine I/O in 8kB units, and silently limits the user-settable parameter io_combine_limit. Version 18 and up only. Changing this parameter causes a service restart. | [optional][default to 16]
**io_method** | Option<**IoMethod**> | EXPERIMENTAL: Controls the maximum number of I/O operations that one process can execute simultaneously. Version 18 and up only. Changing this parameter causes a service restart. (enum: worker, sync, io_uring) | [optional][default to Worker]
**max_stack_depth** | Option<**u32**> | Maximum depth of the stack in bytes. The default is `2097152` (upstream default). | [optional]
**max_parallel_workers** | Option<**u8**> | Sets the maximum number of workers that the system can support for parallel queries. The default is `8` (upstream default). | [optional]
**pg_partman_bgw_role** | Option<**String**> | Controls which role to use for pg_partman's scheduled background tasks. | [optional]
**max_logical_replication_workers** | Option<**u16**> | PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers). The default is `4` (upstream default). Changing this parameter causes a service restart. | [optional]
**max_prepared_transactions** | Option<**u16**> | PostgreSQL maximum prepared transactions. The default is `0`. Changing this parameter causes a service restart. | [optional]
**max_worker_processes** | Option<**u16**> | Sets the maximum number of background processes that the system can support. The default is `8`. Changing this parameter causes a service restart. | [optional]
**pg_stat_statements_track** | Option<**PgStatStatementsTrack**> | Controls which statements are counted. Specify top to track top-level statements (those issued directly by clients), all to also track nested statements (such as statements invoked within functions), or none to disable statement statistics collection. The default is `top`. (enum: all, top, none) | [optional]
**temp_file_limit** | Option<**i32**> | PostgreSQL temporary file limit in KiB, -1 for unlimited | [optional]
**log_error_verbosity** | Option<**LogErrorVerbosity**> | Controls the amount of detail written in the server log for each message that is logged. (enum: TERSE, DEFAULT, VERBOSE) | [optional]
**log_min_duration_statement** | Option<**i32**> | Log statements that take more than this number of milliseconds to run, -1 disables | [optional]
**max_standby_streaming_delay** | Option<**u32**> | Max standby streaming delay in milliseconds. The default is `30000` (upstream default). | [optional]
**jit** | Option<**bool**> | Controls system-wide use of Just-in-Time Compilation (JIT). | [optional]
**max_standby_archive_delay** | Option<**u32**> | Max standby archive delay in milliseconds. The default is `30000` (upstream default). | [optional]
**bg_writer** | Option<[**models::BackgroundBgWriterSettings**](BackgroundBGWriterSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


