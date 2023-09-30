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
pub struct CreateDbaasPgUpgradeCheckRequest {
    /// Target version for upgrade
    #[serde(rename = "target-version")]
    pub target_version: TargetVersion,
}

impl CreateDbaasPgUpgradeCheckRequest {
    pub fn new(target_version: TargetVersion) -> CreateDbaasPgUpgradeCheckRequest {
        CreateDbaasPgUpgradeCheckRequest {
            target_version,
        }
    }
}

/// Target version for upgrade
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TargetVersion {
    #[serde(rename = "14")]
    Variant14,
    #[serde(rename = "15")]
    Variant15,
    #[serde(rename = "12")]
    Variant12,
    #[serde(rename = "13")]
    Variant13,
    #[serde(rename = "11")]
    Variant11,
}

impl Default for TargetVersion {
    fn default() -> TargetVersion {
        Self::Variant14
    }
}

