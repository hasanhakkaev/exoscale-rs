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
pub struct SearchTaskSettings {
    /// The heap usage threshold (as a percentage) required for the sum of heap usages of all search tasks before cancellation is applied. Default is 0.5
    #[serde(rename = "total_heap_percent_threshold", skip_serializing_if = "Option::is_none")]
    pub total_heap_percent_threshold: Option<f64>,
    /// The elapsed time threshold (in milliseconds) required for an individual parent task before it is considered for cancellation. Default is 45000
    #[serde(rename = "elapsed_time_millis_threshold", skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis_threshold: Option<u32>,
    /// The maximum number of search tasks to cancel per millisecond of elapsed time. Default is 0.003
    #[serde(rename = "cancellation_rate", skip_serializing_if = "Option::is_none")]
    pub cancellation_rate: Option<f64>,
    /// The heap usage variance required for an individual parent task before it is considered for cancellation. A task is considered for cancellation when taskHeapUsage is greater than or equal to heapUsageMovingAverage * variance. Default is 2.0
    #[serde(rename = "heap_variance", skip_serializing_if = "Option::is_none")]
    pub heap_variance: Option<f64>,
    /// The window size used to calculate the rolling average of the heap usage for the completed parent tasks. Default is 10
    #[serde(rename = "heap_moving_average_window_size", skip_serializing_if = "Option::is_none")]
    pub heap_moving_average_window_size: Option<u32>,
    /// The maximum number of search tasks to cancel, as a percentage of successful search task completions. Default is 0.1
    #[serde(rename = "cancellation_ratio", skip_serializing_if = "Option::is_none")]
    pub cancellation_ratio: Option<f64>,
    /// The heap usage threshold (as a percentage) required for an individual parent task before it is considered for cancellation. Default is 0.2
    #[serde(rename = "heap_percent_threshold", skip_serializing_if = "Option::is_none")]
    pub heap_percent_threshold: Option<f64>,
    /// The CPU usage threshold (in milliseconds) required for an individual parent task before it is considered for cancellation. Default is 30000
    #[serde(rename = "cpu_time_millis_threshold", skip_serializing_if = "Option::is_none")]
    pub cpu_time_millis_threshold: Option<u32>,
    /// The maximum number of search tasks to cancel in a single iteration of the observer thread. Default is 5.0
    #[serde(rename = "cancellation_burst", skip_serializing_if = "Option::is_none")]
    pub cancellation_burst: Option<f64>,
}

impl SearchTaskSettings {
    pub fn new() -> SearchTaskSettings {
        SearchTaskSettings {
            total_heap_percent_threshold: None,
            elapsed_time_millis_threshold: None,
            cancellation_rate: None,
            heap_variance: None,
            heap_moving_average_window_size: None,
            cancellation_ratio: None,
            heap_percent_threshold: None,
            cpu_time_millis_threshold: None,
            cancellation_burst: None,
        }
    }
}

