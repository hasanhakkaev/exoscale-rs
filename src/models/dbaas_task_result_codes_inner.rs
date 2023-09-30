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
pub struct DbaasTaskResultCodesInner {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dbname", skip_serializing_if = "Option::is_none")]
    pub dbname: Option<String>,
}

impl DbaasTaskResultCodesInner {
    pub fn new() -> DbaasTaskResultCodesInner {
        DbaasTaskResultCodesInner {
            code: None,
            dbname: None,
        }
    }
}


