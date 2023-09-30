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
pub enum EnumMigrationStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "syncing")]
    Syncing,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "done")]
    Done,

}

impl ToString for EnumMigrationStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Running => String::from("running"),
            Self::Syncing => String::from("syncing"),
            Self::Failed => String::from("failed"),
            Self::Done => String::from("done"),
        }
    }
}

impl Default for EnumMigrationStatus {
    fn default() -> EnumMigrationStatus {
        Self::Running
    }
}



