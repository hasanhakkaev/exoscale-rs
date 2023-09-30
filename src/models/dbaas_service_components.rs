/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DbaasServiceComponents : Service component information objects



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceComponents {
    /// Service component name
    #[serde(rename = "component")]
    pub component: String,
    /// DNS name for connecting to the service component
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "kafka-authentication-method", skip_serializing_if = "Option::is_none")]
    pub kafka_authentication_method: Option<crate::models::EnumKafkaAuthMethod>,
    /// Path component of the service URL (useful only if service component is HTTP or HTTPS endpoint)
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port number for connecting to the service component
    #[serde(rename = "port")]
    pub port: i64,
    /// Network access route
    #[serde(rename = "route")]
    pub route: Route,
    /// Whether the endpoint is encrypted or accepts plaintext.                                            By default endpoints are always encrypted and                                            this property is only included for service components that may disable encryption.
    #[serde(rename = "ssl", skip_serializing_if = "Option::is_none")]
    pub ssl: Option<bool>,
    /// DNS usage name
    #[serde(rename = "usage")]
    pub usage: Usage,
}

impl DbaasServiceComponents {
    /// Service component information objects
    pub fn new(component: String, host: String, port: i64, route: Route, usage: Usage) -> DbaasServiceComponents {
        DbaasServiceComponents {
            component,
            host,
            kafka_authentication_method: None,
            path: None,
            port,
            route,
            ssl: None,
            usage,
        }
    }
}

/// Network access route
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Route {
    #[serde(rename = "dynamic")]
    Dynamic,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "privatelink")]
    Privatelink,
}

impl Default for Route {
    fn default() -> Route {
        Self::Dynamic
    }
}
/// DNS usage name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Usage {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "replica")]
    Replica,
}

impl Default for Usage {
    fn default() -> Usage {
        Self::Primary
    }
}
