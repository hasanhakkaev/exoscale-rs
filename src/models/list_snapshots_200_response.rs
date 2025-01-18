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
pub struct ListSnapshots200Response {
    #[serde(rename = "snapshots", skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<models::Snapshot>>,
}

impl ListSnapshots200Response {
    pub fn new() -> ListSnapshots200Response {
        ListSnapshots200Response { snapshots: None }
    }
}
