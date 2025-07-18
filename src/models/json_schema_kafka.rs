/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaKafka {
    /// The (optional) comma-delimited setting for the broker to use to verify that the JWT was issued for one of the expected audiences.
    #[serde(rename = "sasl_oauthbearer_expected_audience", skip_serializing_if = "Option::is_none")]
    pub sasl_oauthbearer_expected_audience: Option<String>,
    /// The maximum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(rename = "group_max_session_timeout_ms", skip_serializing_if = "Option::is_none")]
    pub group_max_session_timeout_ms: Option<u32>,
    /// The number of messages accumulated on a log partition before messages are flushed to disk
    #[serde(rename = "log_flush_interval_messages", skip_serializing_if = "Option::is_none")]
    pub log_flush_interval_messages: Option<u64>,
    /// OIDC JWKS endpoint URL. By setting this the SASL SSL OAuth2/OIDC authentication is enabled. See also other options for SASL OAuth2/OIDC. 
    #[serde(rename = "sasl_oauthbearer_jwks_endpoint_url", skip_serializing_if = "Option::is_none")]
    pub sasl_oauthbearer_jwks_endpoint_url: Option<String>,
    /// The maximum number of connections allowed from each ip address (defaults to 2147483647).
    #[serde(rename = "max_connections_per_ip", skip_serializing_if = "Option::is_none")]
    pub max_connections_per_ip: Option<u32>,
    /// Optional setting for the broker to use to verify that the JWT was created by the expected issuer.
    #[serde(rename = "sasl_oauthbearer_expected_issuer", skip_serializing_if = "Option::is_none")]
    pub sasl_oauthbearer_expected_issuer: Option<String>,
    /// The maximum size in bytes of the offset index
    #[serde(rename = "log_index_size_max_bytes", skip_serializing_if = "Option::is_none")]
    pub log_index_size_max_bytes: Option<u32>,
    /// Enable auto creation of topics
    #[serde(rename = "auto_create_topics_enable", skip_serializing_if = "Option::is_none")]
    pub auto_create_topics_enable: Option<bool>,
    /// The interval with which Kafka adds an entry to the offset index
    #[serde(rename = "log_index_interval_bytes", skip_serializing_if = "Option::is_none")]
    pub log_index_interval_bytes: Option<u32>,
    /// The number of bytes of messages to attempt to fetch for each partition (defaults to 1048576). This is not an absolute maximum, if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made.
    #[serde(rename = "replica_fetch_max_bytes", skip_serializing_if = "Option::is_none")]
    pub replica_fetch_max_bytes: Option<u32>,
    /// Number of partitions for autocreated topics
    #[serde(rename = "num_partitions", skip_serializing_if = "Option::is_none")]
    pub num_partitions: Option<u16>,
    /// The transaction topic segment bytes should be kept relatively small in order to facilitate faster log compaction and cache loads (defaults to 104857600 (100 mebibytes)).
    #[serde(rename = "transaction_state_log_segment_bytes", skip_serializing_if = "Option::is_none")]
    pub transaction_state_log_segment_bytes: Option<u32>,
    /// Maximum bytes expected for the entire fetch response (defaults to 10485760). Records are fetched in batches, and if the first record batch in the first non-empty partition of the fetch is larger than this value, the record batch will still be returned to ensure that progress can be made. As such, this is not an absolute maximum.
    #[serde(rename = "replica_fetch_response_max_bytes", skip_serializing_if = "Option::is_none")]
    pub replica_fetch_response_max_bytes: Option<u32>,
    /// Define whether the timestamp in the message is message create time or log append time.
    #[serde(rename = "log_message_timestamp_type", skip_serializing_if = "Option::is_none")]
    pub log_message_timestamp_type: Option<LogMessageTimestampType>,
    /// Idle connections timeout: the server socket processor threads close the connections that idle for longer than this.
    #[serde(rename = "connections_max_idle_ms", skip_serializing_if = "Option::is_none")]
    pub connections_max_idle_ms: Option<u32>,
    /// The maximum time in ms that a message in any topic is kept in memory before flushed to disk. If not set, the value in log.flush.scheduler.interval.ms is used
    #[serde(rename = "log_flush_interval_ms", skip_serializing_if = "Option::is_none")]
    pub log_flush_interval_ms: Option<u64>,
    /// Should pre allocate file when create new segment?
    #[serde(rename = "log_preallocate", skip_serializing_if = "Option::is_none")]
    pub log_preallocate: Option<bool>,
    /// The amount of time to wait before deleting a file from the filesystem
    #[serde(rename = "log_segment_delete_delay_ms", skip_serializing_if = "Option::is_none")]
    pub log_segment_delete_delay_ms: Option<u32>,
    /// The maximum size of message that the server can receive.
    #[serde(rename = "message_max_bytes", skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<u32>,
    /// The amount of time, in milliseconds, the group coordinator will wait for more consumers to join a new group before performing the first rebalance. A longer delay means potentially fewer rebalances, but increases the time until processing begins. The default value for this is 3 seconds. During development and testing it might be desirable to set this to 0 in order to not delay test execution time.
    #[serde(rename = "group_initial_rebalance_delay_ms", skip_serializing_if = "Option::is_none")]
    pub group_initial_rebalance_delay_ms: Option<u32>,
    /// The maximum size of local log segments that can grow for a partition before it gets eligible for deletion. If set to -2, the value of log.retention.bytes is used. The effective value should always be less than or equal to log.retention.bytes value.
    #[serde(rename = "log_local_retention_bytes", skip_serializing_if = "Option::is_none")]
    pub log_local_retention_bytes: Option<i64>,
    /// The maximum jitter to subtract from logRollTimeMillis (in milliseconds). If not set, the value in log.roll.jitter.hours is used
    #[serde(rename = "log_roll_jitter_ms", skip_serializing_if = "Option::is_none")]
    pub log_roll_jitter_ms: Option<u64>,
    /// The interval at which to remove transactions that have expired due to transactional.id.expiration.ms passing (defaults to 3600000 (1 hour)).
    #[serde(rename = "transaction_remove_expired_transaction_cleanup_interval_ms", skip_serializing_if = "Option::is_none")]
    pub transaction_remove_expired_transaction_cleanup_interval_ms: Option<u32>,
    /// Enable verification that checks that the partition has been added to the transaction before writing transactional records to the partition
    #[serde(rename = "transaction_partition_verification_enable", skip_serializing_if = "Option::is_none")]
    pub transaction_partition_verification_enable: Option<bool>,
    /// Replication factor for autocreated topics
    #[serde(rename = "default_replication_factor", skip_serializing_if = "Option::is_none")]
    pub default_replication_factor: Option<u8>,
    /// The maximum time before a new log segment is rolled out (in milliseconds).
    #[serde(rename = "log_roll_ms", skip_serializing_if = "Option::is_none")]
    pub log_roll_ms: Option<u64>,
    /// The purge interval (in number of requests) of the producer request purgatory(defaults to 1000).
    #[serde(rename = "producer_purgatory_purge_interval_requests", skip_serializing_if = "Option::is_none")]
    pub producer_purgatory_purge_interval_requests: Option<u16>,
    /// The maximum size of the log before deleting messages
    #[serde(rename = "log_retention_bytes", skip_serializing_if = "Option::is_none")]
    pub log_retention_bytes: Option<i64>,
    /// When a producer sets acks to 'all' (or '-1'), min.insync.replicas specifies the minimum number of replicas that must acknowledge a write for the write to be considered successful.
    #[serde(rename = "min_insync_replicas", skip_serializing_if = "Option::is_none")]
    pub min_insync_replicas: Option<u8>,
    /// Specify the final compression type for a given topic. This configuration accepts the standard compression codecs ('gzip', 'snappy', 'lz4', 'zstd'). It additionally accepts 'uncompressed' which is equivalent to no compression; and 'producer' which means retain the original compression codec set by the producer.
    #[serde(rename = "compression_type", skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<CompressionType>,
    /// The maximum difference allowed between the timestamp when a broker receives a message and the timestamp specified in the message
    #[serde(rename = "log_message_timestamp_difference_max_ms", skip_serializing_if = "Option::is_none")]
    pub log_message_timestamp_difference_max_ms: Option<u64>,
    /// The number of milliseconds to keep the local log segments before it gets eligible for deletion. If set to -2, the value of log.retention.ms is used. The effective value should always be less than or equal to log.retention.ms value.
    #[serde(rename = "log_local_retention_ms", skip_serializing_if = "Option::is_none")]
    pub log_local_retention_ms: Option<i64>,
    /// This configuration controls whether down-conversion of message formats is enabled to satisfy consume requests. 
    #[serde(rename = "log_message_downconversion_enable", skip_serializing_if = "Option::is_none")]
    pub log_message_downconversion_enable: Option<bool>,
    /// Name of the scope from which to extract the subject claim from the JWT. Defaults to sub.
    #[serde(rename = "sasl_oauthbearer_sub_claim_name", skip_serializing_if = "Option::is_none")]
    pub sasl_oauthbearer_sub_claim_name: Option<String>,
    /// The maximum number of incremental fetch sessions that the broker will maintain.
    #[serde(rename = "max_incremental_fetch_session_cache_slots", skip_serializing_if = "Option::is_none")]
    pub max_incremental_fetch_session_cache_slots: Option<u16>,
    /// The number of hours to keep a log file before deleting it
    #[serde(rename = "log_retention_hours", skip_serializing_if = "Option::is_none")]
    pub log_retention_hours: Option<i32>,
    /// The minimum allowed session timeout for registered consumers. Longer timeouts give consumers more time to process messages in between heartbeats at the cost of a longer time to detect failures.
    #[serde(rename = "group_min_session_timeout_ms", skip_serializing_if = "Option::is_none")]
    pub group_min_session_timeout_ms: Option<u16>,
    /// The maximum number of bytes in a socket request (defaults to 104857600).
    #[serde(rename = "socket_request_max_bytes", skip_serializing_if = "Option::is_none")]
    pub socket_request_max_bytes: Option<u32>,
    /// The maximum size of a single log file
    #[serde(rename = "log_segment_bytes", skip_serializing_if = "Option::is_none")]
    pub log_segment_bytes: Option<u32>,
    #[serde(rename = "log-cleanup-and-compaction", skip_serializing_if = "Option::is_none")]
    pub log_cleanup_and_compaction: Option<Box<models::ConfigureLogCleanerForTopicCompaction>>,
    /// Log retention window in minutes for offsets topic
    #[serde(rename = "offsets_retention_minutes", skip_serializing_if = "Option::is_none")]
    pub offsets_retention_minutes: Option<u32>,
    /// The number of milliseconds to keep a log file before deleting it (in milliseconds), If not set, the value in log.retention.minutes is used. If set to -1, no time limit is applied.
    #[serde(rename = "log_retention_ms", skip_serializing_if = "Option::is_none")]
    pub log_retention_ms: Option<i64>,
}

impl JsonSchemaKafka {
    pub fn new() -> JsonSchemaKafka {
        JsonSchemaKafka {
            sasl_oauthbearer_expected_audience: None,
            group_max_session_timeout_ms: None,
            log_flush_interval_messages: None,
            sasl_oauthbearer_jwks_endpoint_url: None,
            max_connections_per_ip: None,
            sasl_oauthbearer_expected_issuer: None,
            log_index_size_max_bytes: None,
            auto_create_topics_enable: None,
            log_index_interval_bytes: None,
            replica_fetch_max_bytes: None,
            num_partitions: None,
            transaction_state_log_segment_bytes: None,
            replica_fetch_response_max_bytes: None,
            log_message_timestamp_type: None,
            connections_max_idle_ms: None,
            log_flush_interval_ms: None,
            log_preallocate: None,
            log_segment_delete_delay_ms: None,
            message_max_bytes: None,
            group_initial_rebalance_delay_ms: None,
            log_local_retention_bytes: None,
            log_roll_jitter_ms: None,
            transaction_remove_expired_transaction_cleanup_interval_ms: None,
            transaction_partition_verification_enable: None,
            default_replication_factor: None,
            log_roll_ms: None,
            producer_purgatory_purge_interval_requests: None,
            log_retention_bytes: None,
            min_insync_replicas: None,
            compression_type: None,
            log_message_timestamp_difference_max_ms: None,
            log_local_retention_ms: None,
            log_message_downconversion_enable: None,
            sasl_oauthbearer_sub_claim_name: None,
            max_incremental_fetch_session_cache_slots: None,
            log_retention_hours: None,
            group_min_session_timeout_ms: None,
            socket_request_max_bytes: None,
            log_segment_bytes: None,
            log_cleanup_and_compaction: None,
            offsets_retention_minutes: None,
            log_retention_ms: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogMessageTimestampType {
    #[serde(rename = "CreateTime")]
    CreateTime,
    #[serde(rename = "LogAppendTime")]
    LogAppendTime,
}

impl Default for LogMessageTimestampType {
    fn default() -> LogMessageTimestampType {
        Self::CreateTime
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CompressionType {
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "snappy")]
    Snappy,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "zstd")]
    Zstd,
    #[serde(rename = "uncompressed")]
    Uncompressed,
    #[serde(rename = "producer")]
    Producer,
}

impl Default for CompressionType {
    fn default() -> CompressionType {
        Self::Gzip
    }
}

