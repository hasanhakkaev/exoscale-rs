/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddressRateLimitingSettings {
    /// The number of login attempts allowed before login is blocked
    #[serde(rename = "allowed_tries", skip_serializing_if = "Option::is_none")]
    pub allowed_tries: Option<i32>,
    /// The duration of time that login remains blocked after a failed login
    #[serde(
        rename = "block_expiry_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_expiry_seconds: Option<i32>,
    /// The maximum number of blocked IP addresses
    #[serde(
        rename = "max_blocked_clients",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_blocked_clients: Option<i32>,
    /// The maximum number of tracked IP addresses that have failed login
    #[serde(
        rename = "max_tracked_clients",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_tracked_clients: Option<i32>,
    /// The window of time in which the value for `allowed_tries` is enforced
    #[serde(
        rename = "time_window_seconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_window_seconds: Option<i32>,
    /// The type of rate limiting
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl IpAddressRateLimitingSettings {
    pub fn new() -> IpAddressRateLimitingSettings {
        IpAddressRateLimitingSettings {
            allowed_tries: None,
            block_expiry_seconds: None,
            max_blocked_clients: None,
            max_tracked_clients: None,
            time_window_seconds: None,
            r#type: None,
        }
    }
}
/// The type of rate limiting
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ip
    }
}
