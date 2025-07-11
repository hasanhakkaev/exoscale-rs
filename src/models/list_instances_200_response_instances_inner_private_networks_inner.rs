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
pub struct ListInstances200ResponseInstancesInnerPrivateNetworksInner {
    /// Private Network ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Private Network MAC address
    #[serde(rename = "mac-address", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

impl ListInstances200ResponseInstancesInnerPrivateNetworksInner {
    /// Private Network
    pub fn new() -> ListInstances200ResponseInstancesInnerPrivateNetworksInner {
        ListInstances200ResponseInstancesInnerPrivateNetworksInner {
            id: None,
            mac_address: None,
        }
    }
}

