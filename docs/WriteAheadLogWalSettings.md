# WriteAheadLogWalSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_slot_wal_keep_size** | Option<**i32**> | PostgreSQL maximum WAL size (MB) reserved for replication slots. Default is -1 (unlimited). wal_keep_size minimum WAL size setting takes precedence over this. | [optional]
**max_wal_senders** | Option<**i32**> | PostgreSQL maximum WAL senders | [optional]
**wal_sender_timeout** | Option<**i32**> | Terminate replication connections that are inactive for longer than this amount of time, in milliseconds. | [optional]
**wal_writer_delay** | Option<**i32**> | WAL flush interval in milliseconds. Note that setting this value to lower than the default 200ms may negatively impact performance | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


