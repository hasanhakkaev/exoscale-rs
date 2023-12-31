/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnumMigrationMethod {
    #[serde(rename = "dump")]
    Dump,
    #[serde(rename = "replication")]
    Replication,

}

impl ToString for EnumMigrationMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Dump => String::from("dump"),
            Self::Replication => String::from("replication"),
        }
    }
}

impl Default for EnumMigrationMethod {
    fn default() -> EnumMigrationMethod {
        Self::Dump
    }
}




