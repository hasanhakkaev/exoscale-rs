/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasDatadogTag {
    /// Optional tag explanation
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Tag value
    #[serde(rename = "tag")]
    pub tag: String,
}

impl DbaasDatadogTag {
    pub fn new(tag: String) -> DbaasDatadogTag {
        DbaasDatadogTag { comment: None, tag }
    }
}
