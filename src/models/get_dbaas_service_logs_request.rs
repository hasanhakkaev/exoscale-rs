/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDbaasServiceLogsRequest {
    /// How many log entries to receive at most, up to 500 (default: 100)
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "sort-order", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<crate::models::EnumSortOrder>,
    /// Opaque offset identifier
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

impl GetDbaasServiceLogsRequest {
    pub fn new() -> GetDbaasServiceLogsRequest {
        GetDbaasServiceLogsRequest {
            limit: None,
            sort_order: None,
            offset: None,
        }
    }
}


