/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */
/// UpdateDbaasServiceOpensearchRequestIndexTemplate : Template settings for all new indexes
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDbaasServiceOpensearchRequestIndexTemplate {
    /// The maximum number of nested JSON objects that a single document can contain across all nested types. This limit helps to prevent out of memory errors when a document contains too many nested objects. Default is 10000.
    #[serde(
        rename = "mapping-nested-objects-limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping_nested_objects_limit: Option<i64>,
    /// The number of replicas each primary shard has.
    #[serde(rename = "number-of-replicas", skip_serializing_if = "Option::is_none")]
    pub number_of_replicas: Option<i64>,
    /// The number of primary shards that an index should have.
    #[serde(rename = "number-of-shards", skip_serializing_if = "Option::is_none")]
    pub number_of_shards: Option<i64>,
}

impl UpdateDbaasServiceOpensearchRequestIndexTemplate {
    /// Template settings for all new indexes
    pub fn new() -> UpdateDbaasServiceOpensearchRequestIndexTemplate {
        UpdateDbaasServiceOpensearchRequestIndexTemplate {
            mapping_nested_objects_limit: None,
            number_of_replicas: None,
            number_of_shards: None,
        }
    }
}
