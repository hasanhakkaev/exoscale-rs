# JsonSchemaMysql

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**net_write_timeout** | Option<**u16**> | The number of seconds to wait for a block to be written to a connection before aborting the write. | [optional]
**internal_tmp_mem_storage_engine** | Option<**String**> | The storage engine for in-memory internal temporary tables. | [optional]
**sql_mode** | Option<**String**> | Global SQL mode. Set to empty to use MySQL server defaults. When creating a new service and not setting this field Aiven default SQL mode (strict, SQL standard compliant) will be assigned. | [optional]
**information_schema_stats_expiry** | Option<**u32**> | The time, in seconds, before cached statistics expire | [optional]
**sort_buffer_size** | Option<**u32**> | Sort buffer size in bytes for ORDER BY optimization. Default is 262144 (256K) | [optional]
**innodb_thread_concurrency** | Option<**u16**> | Defines the maximum number of threads permitted inside of InnoDB. Default is 0 (infinite concurrency - no limit) | [optional]
**innodb_write_io_threads** | Option<**u8**> | The number of I/O threads for write operations in InnoDB. Default is 4. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**innodb_ft_min_token_size** | Option<**u8**> | Minimum length of words that are stored in an InnoDB FULLTEXT index. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**innodb_change_buffer_max_size** | Option<**u8**> | Maximum size for the InnoDB change buffer, as a percentage of the total size of the buffer pool. Default is 25 | [optional]
**innodb_flush_neighbors** | Option<**u8**> | Specifies whether flushing a page from the InnoDB buffer pool also flushes other dirty pages in the same extent (default is 1): 0 - dirty pages in the same extent are not flushed, 1 - flush contiguous dirty pages in the same extent, 2 - flush dirty pages in the same extent | [optional]
**tmp_table_size** | Option<**u32**> | Limits the size of internal in-memory tables. Also set max_heap_table_size. Default is 16777216 (16M) | [optional]
**slow_query_log** | Option<**bool**> | Slow query log enables capturing of slow queries. Setting slow_query_log to false also truncates the mysql.slow_log table. Default is off | [optional]
**connect_timeout** | Option<**u16**> | The number of seconds that the mysqld server waits for a connect packet before responding with Bad handshake | [optional]
**log_output** | Option<**String**> | The slow log output destination when slow_query_log is ON. To enable MySQL AI Insights, choose INSIGHTS. To use MySQL AI Insights and the mysql.slow_log table at the same time, choose INSIGHTS,TABLE. To only use the mysql.slow_log table, choose TABLE. To silence slow logs, choose NONE. | [optional]
**net_read_timeout** | Option<**u16**> | The number of seconds to wait for more data from a connection before aborting the read. | [optional]
**innodb_lock_wait_timeout** | Option<**u16**> | The length of time in seconds an InnoDB transaction waits for a row lock before giving up. Default is 120. | [optional]
**wait_timeout** | Option<**u32**> | The number of seconds the server waits for activity on a noninteractive connection before closing it. | [optional]
**innodb_rollback_on_timeout** | Option<**bool**> | When enabled a transaction timeout causes InnoDB to abort and roll back the entire transaction. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**group_concat_max_len** | Option<**u64**> | The maximum permitted result length in bytes for the GROUP_CONCAT() function. | [optional]
**net_buffer_length** | Option<**u32**> | Start sizes of connection buffer and result buffer. Default is 16384 (16K). Changing this parameter will lead to a restart of the MySQL service. | [optional]
**innodb_print_all_deadlocks** | Option<**bool**> | When enabled, information about all deadlocks in InnoDB user transactions is recorded in the error log. Disabled by default. | [optional]
**innodb_online_alter_log_max_size** | Option<**u64**> | The upper limit in bytes on the size of the temporary log files used during online DDL operations for InnoDB tables. | [optional]
**interactive_timeout** | Option<**u32**> | The number of seconds the server waits for activity on an interactive connection before closing it. | [optional]
**innodb_log_buffer_size** | Option<**u32**> | The size in bytes of the buffer that InnoDB uses to write to the log files on disk. | [optional]
**max_allowed_packet** | Option<**u32**> | Size of the largest message in bytes that can be received by the server. Default is 67108864 (64M) | [optional]
**max_heap_table_size** | Option<**u32**> | Limits the size of internal in-memory tables. Also set tmp_table_size. Default is 16777216 (16M) | [optional]
**innodb_ft_server_stopword_table** | Option<**String**> | This option is used to specify your own InnoDB FULLTEXT index stopword list for all InnoDB tables. | [optional]
**innodb_read_io_threads** | Option<**u8**> | The number of I/O threads for read operations in InnoDB. Default is 4. Changing this parameter will lead to a restart of the MySQL service. | [optional]
**sql_require_primary_key** | Option<**bool**> | Require primary key to be defined for new tables or old tables modified with ALTER TABLE and fail if missing. It is recommended to always have primary keys because various functionality may break if any large table is missing them. | [optional]
**default_time_zone** | Option<**String**> | Default server time zone as an offset from UTC (from -12:00 to +12:00), a time zone name, or 'SYSTEM' to use the MySQL server default. | [optional]
**long_query_time** | Option<**f64**> | The slow_query_logs work as SQL statements that take more than long_query_time seconds to execute. Default is 10s | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


