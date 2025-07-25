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
pub struct ListDbaasExternalEndpointTypes200ResponseEndpointTypesInner {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::EnumExternalEndpointTypes>,
    #[serde(rename = "service-types", skip_serializing_if = "Option::is_none")]
    pub service_types: Option<Vec<String>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListDbaasExternalEndpointTypes200ResponseEndpointTypesInner {
    pub fn new() -> ListDbaasExternalEndpointTypes200ResponseEndpointTypesInner {
        ListDbaasExternalEndpointTypes200ResponseEndpointTypesInner {
            r#type: None,
            service_types: None,
            title: None,
        }
    }
}

