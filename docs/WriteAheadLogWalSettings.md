# WriteAheadLogWalSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_slot_wal_keep_size** | Option<**i32**> | PostgreSQL maximum WAL size (MB) reserved for replication slots. If `-1` is specified, replication slots may retain an unlimited amount of WAL files. The default is `-1` (upstream default). wal_keep_size minimum WAL size setting takes precedence over this. | [optional]
**max_wal_senders** | Option<**u16**> | PostgreSQL maximum WAL senders. The default is `20`. Changing this parameter causes a service restart. | [optional]
**wal_sender_timeout** | Option<**u32**> | Terminate replication connections that are inactive for longer than this amount of time, in milliseconds. | [optional]
**wal_writer_delay** | Option<**u8**> | WAL flush interval in milliseconds. The default is `200`. Setting this parameter to a lower value may negatively impact performance. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


