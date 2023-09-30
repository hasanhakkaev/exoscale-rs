# DbaasServicePg

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pgbouncer_settings** | Option<[**serde_json::Value**](.md)> | PGBouncer connection pooling settings | [optional]
**updated_at** | Option<**String**> | Service last update timestamp (ISO 8601) | [optional]
**node_count** | Option<**i64**> | Number of service nodes in the active plan | [optional]
**connection_info** | Option<[**crate::models::DbaasServicePgConnectionInfo**](dbaas_service_pg_connection_info.md)> |  | [optional]
**backup_schedule** | Option<[**crate::models::DbaasServiceMysqlBackupSchedule**](dbaas_service_mysql_backup_schedule.md)> |  | [optional]
**node_cpu_count** | Option<**i64**> | Number of CPUs for each node | [optional]
**integrations** | Option<[**Vec<crate::models::DbaasIntegration>**](dbaas-integration.md)> | Service integrations | [optional]
**zone** | Option<**String**> | The zone where the service is running | [optional]
**node_states** | Option<[**Vec<crate::models::DbaasNodeState>**](dbaas-node-state.md)> | State of individual service nodes | [optional]
**name** | **String** |  | 
**connection_pools** | Option<[**Vec<crate::models::DbaasServicePgConnectionPoolsInner>**](dbaas_service_pg_connection_pools_inner.md)> | PostgreSQL PGBouncer connection pools | [optional]
**r#type** | **String** |  | 
**state** | Option<[**crate::models::EnumServiceState**](enum-service-state.md)> |  | [optional]
**timescaledb_settings** | Option<[**serde_json::Value**](.md)> | TimescaleDB extension configuration values | [optional]
**databases** | Option<**Vec<String>**> | List of PostgreSQL databases | [optional]
**ip_filter** | Option<**Vec<String>**> | Allowed CIDR address blocks for incoming connections | [optional]
**backups** | Option<[**Vec<crate::models::DbaasServiceBackup>**](dbaas-service-backup.md)> | List of backups for the service | [optional]
**termination_protection** | Option<**bool**> | Service is protected against termination and powering off | [optional]
**notifications** | Option<[**Vec<crate::models::DbaasServiceNotification>**](dbaas-service-notification.md)> | Service notifications | [optional]
**components** | Option<[**Vec<crate::models::DbaasServiceMysqlComponentsInner>**](dbaas_service_mysql_components_inner.md)> | Service component information objects | [optional]
**synchronous_replication** | Option<[**crate::models::EnumPgSynchronousReplication**](enum-pg-synchronous-replication.md)> |  | [optional]
**pglookout_settings** | Option<[**serde_json::Value**](.md)> | PGLookout settings | [optional]
**maintenance** | Option<[**crate::models::DbaasServiceMaintenance**](dbaas-service-maintenance.md)> |  | [optional]
**disk_size** | Option<**i64**> | TODO UNIT disk space for data storage | [optional]
**node_memory** | Option<**i64**> | TODO UNIT of memory for each node | [optional]
**uri** | Option<**String**> | URI for connecting to the service (may be absent) | [optional]
**uri_params** | Option<[**serde_json::Value**](.md)> | service_uri parameterized into key-value pairs | [optional]
**version** | Option<**String**> | PostgreSQL version | [optional]
**created_at** | Option<**String**> | Service creation timestamp (ISO 8601) | [optional]
**plan** | **String** | Subscription plan | 
**work_mem** | Option<**i64**> | Sets the maximum amount of memory to be used by a query operation (such as a sort or hash table) before writing to temporary disk files, in MB. Default is 1MB + 0.075% of total RAM (up to 32MB). | [optional]
**shared_buffers_percentage** | Option<**i64**> | Percentage of total RAM that the database server uses for shared memory buffers. Valid range is 20-60 (float), which corresponds to 20% - 60%. This setting adjusts the shared_buffers configuration value. | [optional]
**pg_settings** | Option<[**serde_json::Value**](.md)> | PostgreSQL-specific settings | [optional]
**max_connections** | Option<**i64**> | Maximum number of connections allowed to an instance | [optional]
**users** | Option<[**Vec<crate::models::DbaasServicePgUsersInner>**](dbaas_service_pg_users_inner.md)> | List of service users | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


