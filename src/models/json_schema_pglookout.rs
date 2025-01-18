/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */
/// JsonSchemaPglookout : System-wide settings for pglookout.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaPglookout {
    /// Number of seconds of master unavailability before triggering database failover to standby
    #[serde(
        rename = "max_failover_replication_time_lag",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_failover_replication_time_lag: Option<i32>,
}

impl JsonSchemaPglookout {
    /// System-wide settings for pglookout.
    pub fn new() -> JsonSchemaPglookout {
        JsonSchemaPglookout {
            max_failover_replication_time_lag: None,
        }
    }
}
