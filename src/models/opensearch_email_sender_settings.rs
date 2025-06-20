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
pub struct OpensearchEmailSenderSettings {
    /// This should be identical to the Sender name defined in Opensearch dashboards
    #[serde(rename = "email_sender_name")]
    pub email_sender_name: String,
    /// Sender password for Opensearch alerts to authenticate with SMTP server
    #[serde(rename = "email_sender_password")]
    pub email_sender_password: String,
    #[serde(rename = "email_sender_username")]
    pub email_sender_username: String,
}

impl OpensearchEmailSenderSettings {
    pub fn new(email_sender_name: String, email_sender_password: String, email_sender_username: String) -> OpensearchEmailSenderSettings {
        OpensearchEmailSenderSettings {
            email_sender_name,
            email_sender_password,
            email_sender_username,
        }
    }
}

