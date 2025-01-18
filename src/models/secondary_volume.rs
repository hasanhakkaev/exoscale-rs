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

/// SecondaryVolume : Secondary Volume

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecondaryVolume {
    /// Volume ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Volume name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Volume creation date
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Volume state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Volume size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Volume block size
    #[serde(rename = "blocksize", skip_serializing_if = "Option::is_none")]
    pub blocksize: Option<i64>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<models::InstanceTarget>>,
}

impl SecondaryVolume {
    #[allow(dead_code)]
    /// Secondary Volume
    pub fn new() -> SecondaryVolume {
        SecondaryVolume {
            id: None,
            name: None,
            created_at: None,
            state: None,
            size: None,
            blocksize: None,
            instance: None,
        }
    }
}

/// Volume state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "partially-detached")]
    PartiallyDetached,
    #[serde(rename = "snapshotting-detached")]
    SnapshottingDetached,
    #[serde(rename = "creating-metadata")]
    CreatingMetadata,
    #[serde(rename = "purging")]
    Purging,
    #[serde(rename = "detached")]
    Detached,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "attaching")]
    Attaching,
    #[serde(rename = "promoting")]
    Promoting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "snapshotting-attached")]
    SnapshottingAttached,
    #[serde(rename = "partially-attached")]
    PartiallyAttached,
    #[serde(rename = "attached")]
    Attached,
    #[serde(rename = "allocated")]
    Allocated,
    #[serde(rename = "detaching")]
    Detaching,
}

impl Default for State {
    fn default() -> State {
        Self::Deleted
    }
}
