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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasEndpointRsyslog {
    /// Rsyslog server IP address or hostname
    #[serde(rename = "server")]
    pub server: String,
    /// Rsyslog server port
    #[serde(rename = "port")]
    pub port: i64,
    /// Require TLS
    #[serde(rename = "tls")]
    pub tls: bool,
    #[serde(rename = "format")]
    pub format: models::EnumRsyslogFormat,
    /// Custom syslog message format
    #[serde(rename = "logline", skip_serializing_if = "Option::is_none")]
    pub logline: Option<String>,
    /// Structured data block for log message
    #[serde(rename = "sd", skip_serializing_if = "Option::is_none")]
    pub sd: Option<String>,
    /// Rsyslog max message size
    #[serde(rename = "max-message-size", skip_serializing_if = "Option::is_none")]
    pub max_message_size: Option<i64>,
}

impl DbaasEndpointRsyslog {
    pub fn new(
        server: String,
        port: i64,
        tls: bool,
        format: models::EnumRsyslogFormat,
    ) -> DbaasEndpointRsyslog {
        DbaasEndpointRsyslog {
            server,
            port,
            tls,
            format,
            logline: None,
            sd: None,
            max_message_size: None,
        }
    }
}
