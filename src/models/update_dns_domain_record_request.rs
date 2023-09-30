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
pub struct UpdateDnsDomainRecordRequest {
    /// DNS domain record name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// DNS domain record content
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// DNS domain record TTL
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// DNS domain record priority
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

impl UpdateDnsDomainRecordRequest {
    pub fn new() -> UpdateDnsDomainRecordRequest {
        UpdateDnsDomainRecordRequest {
            name: None,
            content: None,
            ttl: None,
            priority: None,
        }
    }
}


