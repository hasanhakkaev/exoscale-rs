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

/// DbaasServiceMaintenance : Automatic maintenance settings
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DbaasServiceMaintenance {
    /// Day of week for installing updates
    #[serde(rename = "dow")]
    pub dow: Dow,
    /// Time for installing updates, UTC
    #[serde(rename = "time")]
    pub time: String,
    /// List of updates waiting to be installed
    #[serde(rename = "updates")]
    pub updates: Vec<models::DbaasServiceUpdate>,
}

impl DbaasServiceMaintenance {
    /// Automatic maintenance settings
    pub fn new(
        dow: Dow,
        time: String,
        updates: Vec<models::DbaasServiceUpdate>,
    ) -> DbaasServiceMaintenance {
        DbaasServiceMaintenance { dow, time, updates }
    }
}
/// Day of week for installing updates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Dow {
    #[serde(rename = "saturday")]
    Saturday,
    #[serde(rename = "tuesday")]
    Tuesday,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "wednesday")]
    Wednesday,
    #[serde(rename = "sunday")]
    Sunday,
    #[serde(rename = "friday")]
    Friday,
    #[serde(rename = "monday")]
    Monday,
    #[serde(rename = "thursday")]
    Thursday,
}

impl Default for Dow {
    fn default() -> Dow {
        Self::Saturday
    }
}
