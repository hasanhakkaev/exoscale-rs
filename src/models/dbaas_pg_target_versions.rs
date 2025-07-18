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
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DbaasPgTargetVersions {
    #[serde(rename = "14")]
    Variant14,
    #[serde(rename = "17")]
    Variant17,
    #[serde(rename = "15")]
    Variant15,
    #[serde(rename = "13")]
    Variant13,
    #[serde(rename = "16")]
    Variant16,

}

impl std::fmt::Display for DbaasPgTargetVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Variant14 => write!(f, "14"),
            Self::Variant17 => write!(f, "17"),
            Self::Variant15 => write!(f, "15"),
            Self::Variant13 => write!(f, "13"),
            Self::Variant16 => write!(f, "16"),
        }
    }
}

impl Default for DbaasPgTargetVersions {
    fn default() -> DbaasPgTargetVersions {
        Self::Variant14
    }
}

