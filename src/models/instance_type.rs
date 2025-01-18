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

/// InstanceType : Compute instance type
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstanceType {
    /// Instance type ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Instance type size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    /// Instance type family
    #[serde(rename = "family", skip_serializing_if = "Option::is_none")]
    pub family: Option<Family>,
    /// CPU count
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<i64>,
    /// GPU count
    #[serde(rename = "gpus", skip_serializing_if = "Option::is_none")]
    pub gpus: Option<i64>,
    /// Requires authorization or publicly available
    #[serde(rename = "authorized", skip_serializing_if = "Option::is_none")]
    pub authorized: Option<bool>,
    /// Available memory
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Instance Type available zones
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<models::ZoneName>>,
}

impl InstanceType {
    /// Compute instance type
    pub fn new() -> InstanceType {
        InstanceType {
            id: None,
            size: None,
            family: None,
            cpus: None,
            gpus: None,
            authorized: None,
            memory: None,
            zones: None,
        }
    }
}
/// Instance type size
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Size {
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "huge")]
    Huge,
    #[serde(rename = "jumbo")]
    Jumbo,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "mega")]
    Mega,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "extra-large")]
    ExtraLarge,
    #[serde(rename = "titan")]
    Titan,
    #[serde(rename = "micro")]
    Micro,
    #[serde(rename = "colossus")]
    Colossus,
    #[serde(rename = "tiny")]
    Tiny,
}

impl Default for Size {
    fn default() -> Size {
        Self::Large
    }
}
/// Instance type family
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Family {
    #[serde(rename = "gpu3")]
    Gpu3,
    #[serde(rename = "gpu3080ti")]
    Gpu3080ti,
    #[serde(rename = "gpu2")]
    Gpu2,
    #[serde(rename = "gpu")]
    Gpu,
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "gpua5000")]
    Gpua5000,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "colossus")]
    Colossus,
    #[serde(rename = "cpu")]
    Cpu,
}

impl Default for Family {
    fn default() -> Family {
        Self::Gpu3
    }
}
