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
pub struct JsonSchemaPg {
    /// Specifies the number of bytes reserved to track the currently executing command for each active session.
    #[serde(rename = "track_activity_query_size", skip_serializing_if = "Option::is_none")]
    pub track_activity_query_size: Option<u16>,
    /// PostgreSQL service timezone
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Enables timing of database I/O calls. This parameter is off by default, because it will repeatedly query the operating system for the current time, which may cause significant overhead on some platforms.
    #[serde(rename = "track_io_timing", skip_serializing_if = "Option::is_none")]
    pub track_io_timing: Option<TrackIoTiming>,
    /// Enables or disables query plan monitoring
    #[serde(rename = "pg_stat_monitor.pgsm_enable_query_plan", skip_serializing_if = "Option::is_none")]
    pub pg_stat_monitor_period_pgsm_enable_query_plan: Option<bool>,
    /// PostgreSQL maximum number of files that can be open per process
    #[serde(rename = "max_files_per_process", skip_serializing_if = "Option::is_none")]
    pub max_files_per_process: Option<u16>,
    /// Sets the maximum number of buckets 
    #[serde(rename = "pg_stat_monitor.pgsm_max_buckets", skip_serializing_if = "Option::is_none")]
    pub pg_stat_monitor_period_pgsm_max_buckets: Option<u8>,
    #[serde(rename = "wal", skip_serializing_if = "Option::is_none")]
    pub wal: Option<Box<models::WriteAheadLogWalSettings>>,
    /// Specifies the default TOAST compression method for values of compressible columns (the default is lz4).
    #[serde(rename = "default_toast_compression", skip_serializing_if = "Option::is_none")]
    pub default_toast_compression: Option<DefaultToastCompression>,
    /// This is the amount of time, in milliseconds, to wait on a lock before checking to see if there is a deadlock condition.
    #[serde(rename = "deadlock_timeout", skip_serializing_if = "Option::is_none")]
    pub deadlock_timeout: Option<u32>,
    /// Time out sessions with open transactions after this number of milliseconds
    #[serde(rename = "idle_in_transaction_session_timeout", skip_serializing_if = "Option::is_none")]
    pub idle_in_transaction_session_timeout: Option<u32>,
    /// PostgreSQL maximum predicate locks per transaction
    #[serde(rename = "max_pred_locks_per_transaction", skip_serializing_if = "Option::is_none")]
    pub max_pred_locks_per_transaction: Option<u16>,
    /// PostgreSQL maximum replication slots
    #[serde(rename = "max_replication_slots", skip_serializing_if = "Option::is_none")]
    pub max_replication_slots: Option<u8>,
    #[serde(rename = "autovacuum", skip_serializing_if = "Option::is_none")]
    pub autovacuum: Option<Box<models::AutovacuumSettings>>,
    /// Sets the maximum number of workers that can be started by a single Gather or Gather Merge node
    #[serde(rename = "max_parallel_workers_per_gather", skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers_per_gather: Option<u8>,
    /// Sets the time interval to run pg_partman's scheduled tasks
    #[serde(rename = "pg_partman_bgw.interval", skip_serializing_if = "Option::is_none")]
    pub pg_partman_bgw_period_interval: Option<u32>,
    /// Choose from one of the available log-formats. These can support popular log analyzers like pgbadger, pganalyze etc.
    #[serde(rename = "log_line_prefix", skip_serializing_if = "Option::is_none")]
    pub log_line_prefix: Option<LogLinePrefix>,
    /// Log statements for each temporary file created larger than this number of kilobytes, -1 disables
    #[serde(rename = "log_temp_files", skip_serializing_if = "Option::is_none")]
    pub log_temp_files: Option<i32>,
    /// PostgreSQL maximum locks per transaction
    #[serde(rename = "max_locks_per_transaction", skip_serializing_if = "Option::is_none")]
    pub max_locks_per_transaction: Option<u16>,
    /// Record commit time of transactions.
    #[serde(rename = "track_commit_timestamp", skip_serializing_if = "Option::is_none")]
    pub track_commit_timestamp: Option<TrackCommitTimestamp>,
    /// Enables tracking of function call counts and time used.
    #[serde(rename = "track_functions", skip_serializing_if = "Option::is_none")]
    pub track_functions: Option<TrackFunctions>,
    /// Maximum depth of the stack in bytes
    #[serde(rename = "max_stack_depth", skip_serializing_if = "Option::is_none")]
    pub max_stack_depth: Option<u32>,
    /// Sets the maximum number of workers that the system can support for parallel queries
    #[serde(rename = "max_parallel_workers", skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers: Option<u8>,
    /// Controls which role to use for pg_partman's scheduled background tasks.
    #[serde(rename = "pg_partman_bgw.role", skip_serializing_if = "Option::is_none")]
    pub pg_partman_bgw_period_role: Option<String>,
    /// PostgreSQL maximum logical replication workers (taken from the pool of max_parallel_workers)
    #[serde(rename = "max_logical_replication_workers", skip_serializing_if = "Option::is_none")]
    pub max_logical_replication_workers: Option<u8>,
    /// PostgreSQL maximum prepared transactions
    #[serde(rename = "max_prepared_transactions", skip_serializing_if = "Option::is_none")]
    pub max_prepared_transactions: Option<u16>,
    /// Sets the maximum number of background processes that the system can support
    #[serde(rename = "max_worker_processes", skip_serializing_if = "Option::is_none")]
    pub max_worker_processes: Option<u8>,
    /// Controls which statements are counted. Specify top to track top-level statements (those issued directly by clients), all to also track nested statements (such as statements invoked within functions), or none to disable statement statistics collection. The default value is top.
    #[serde(rename = "pg_stat_statements.track", skip_serializing_if = "Option::is_none")]
    pub pg_stat_statements_period_track: Option<PgStatStatementsPeriodTrack>,
    /// PostgreSQL temporary file limit in KiB, -1 for unlimited
    #[serde(rename = "temp_file_limit", skip_serializing_if = "Option::is_none")]
    pub temp_file_limit: Option<i32>,
    /// Controls the amount of detail written in the server log for each message that is logged.
    #[serde(rename = "log_error_verbosity", skip_serializing_if = "Option::is_none")]
    pub log_error_verbosity: Option<LogErrorVerbosity>,
    /// Log statements that take more than this number of milliseconds to run, -1 disables
    #[serde(rename = "log_min_duration_statement", skip_serializing_if = "Option::is_none")]
    pub log_min_duration_statement: Option<i32>,
    /// Max standby streaming delay in milliseconds
    #[serde(rename = "max_standby_streaming_delay", skip_serializing_if = "Option::is_none")]
    pub max_standby_streaming_delay: Option<u32>,
    /// Controls system-wide use of Just-in-Time Compilation (JIT).
    #[serde(rename = "jit", skip_serializing_if = "Option::is_none")]
    pub jit: Option<bool>,
    /// Max standby archive delay in milliseconds
    #[serde(rename = "max_standby_archive_delay", skip_serializing_if = "Option::is_none")]
    pub max_standby_archive_delay: Option<u32>,
    #[serde(rename = "bg-writer", skip_serializing_if = "Option::is_none")]
    pub bg_writer: Option<Box<models::BackgroundBgWriterSettings>>,
}

impl JsonSchemaPg {
    pub fn new() -> JsonSchemaPg {
        JsonSchemaPg {
            track_activity_query_size: None,
            timezone: None,
            track_io_timing: None,
            pg_stat_monitor_period_pgsm_enable_query_plan: None,
            max_files_per_process: None,
            pg_stat_monitor_period_pgsm_max_buckets: None,
            wal: None,
            default_toast_compression: None,
            deadlock_timeout: None,
            idle_in_transaction_session_timeout: None,
            max_pred_locks_per_transaction: None,
            max_replication_slots: None,
            autovacuum: None,
            max_parallel_workers_per_gather: None,
            pg_partman_bgw_period_interval: None,
            log_line_prefix: None,
            log_temp_files: None,
            max_locks_per_transaction: None,
            track_commit_timestamp: None,
            track_functions: None,
            max_stack_depth: None,
            max_parallel_workers: None,
            pg_partman_bgw_period_role: None,
            max_logical_replication_workers: None,
            max_prepared_transactions: None,
            max_worker_processes: None,
            pg_stat_statements_period_track: None,
            temp_file_limit: None,
            log_error_verbosity: None,
            log_min_duration_statement: None,
            max_standby_streaming_delay: None,
            jit: None,
            max_standby_archive_delay: None,
            bg_writer: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackIoTiming {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackIoTiming {
    fn default() -> TrackIoTiming {
        Self::Off
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultToastCompression {
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "pglz")]
    Pglz,
}

impl Default for DefaultToastCompression {
    fn default() -> DefaultToastCompression {
        Self::Lz4
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLinePrefix {
    #[serde(rename = "'pid=%p,user=%u,db=%d,app=%a,client=%h '")]
    QuotePidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentHQuote,
    #[serde(rename = "'%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h '")]
    QuotePercentTLeftSquareBracketPercentPRightSquareBracketColonLeftSquareBracketPercentL1RightSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentHQuote,
    #[serde(rename = "'%m [%p] %q[user=%u,db=%d,app=%a] '")]
    QuotePercentMLeftSquareBracketPercentPRightSquareBracketPercentQLeftSquareBracketUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentARightSquareBracketQuote,
}

impl Default for LogLinePrefix {
    fn default() -> LogLinePrefix {
        Self::QuotePidEqualPercentPCommaUserEqualPercentUCommaDbEqualPercentDCommaAppEqualPercentACommaClientEqualPercentHQuote
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackCommitTimestamp {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
}

impl Default for TrackCommitTimestamp {
    fn default() -> TrackCommitTimestamp {
        Self::Off
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrackFunctions {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "none")]
    None,
}

impl Default for TrackFunctions {
    fn default() -> TrackFunctions {
        Self::All
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PgStatStatementsPeriodTrack {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "none")]
    None,
}

impl Default for PgStatStatementsPeriodTrack {
    fn default() -> PgStatStatementsPeriodTrack {
        Self::All
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogErrorVerbosity {
    #[serde(rename = "TERSE")]
    Terse,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "VERBOSE")]
    Verbose,
}

impl Default for LogErrorVerbosity {
    fn default() -> LogErrorVerbosity {
        Self::Terse
    }
}

