# JsonSchemaValkey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ssl** | Option<**bool**> |  | [optional][default to true]
**lfu_log_factor** | Option<**u8**> |  | [optional][default to 10]
**maxmemory_policy** | Option<**String**> |  | [optional][default to Noeviction]
**io_threads** | Option<**u8**> | Set Valkey IO thread count. Changing this will cause a restart of the Valkey service. | [optional]
**lfu_decay_time** | Option<**u8**> |  | [optional][default to 1]
**pubsub_client_output_buffer_limit** | Option<**u16**> | Set output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan. | [optional]
**notify_keyspace_events** | Option<**String**> |  | [optional][default to ]
**persistence** | Option<**String**> | When persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked. | [optional]
**timeout** | Option<**u32**> |  | [optional][default to 300]
**acl_channels_default** | Option<**String**> | Determines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default. | [optional]
**number_of_databases** | Option<**u8**> | Set number of Valkey databases. Changing this will cause a restart of the Valkey service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


