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
pub enum InstanceState {
    #[serde(rename = "expunging")]
    Expunging,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "destroying")]
    Destroying,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "migrating")]
    Migrating,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "destroyed")]
    Destroyed,

}

impl ToString for InstanceState {
    fn to_string(&self) -> String {
        match self {
            Self::Expunging => String::from("expunging"),
            Self::Starting => String::from("starting"),
            Self::Destroying => String::from("destroying"),
            Self::Running => String::from("running"),
            Self::Stopping => String::from("stopping"),
            Self::Stopped => String::from("stopped"),
            Self::Migrating => String::from("migrating"),
            Self::Error => String::from("error"),
            Self::Destroyed => String::from("destroyed"),
        }
    }
}

impl Default for InstanceState {
    fn default() -> InstanceState {
        Self::Expunging
    }
}




