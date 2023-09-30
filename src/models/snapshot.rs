/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Snapshot : Snapshot



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Snapshot ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Snapshot name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Snapshot creation date
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Snapshot state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Snapshot size in GB
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "export", skip_serializing_if = "Option::is_none")]
    pub export: Option<Box<crate::models::SnapshotExport>>,
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<crate::models::Instance>>,
}

impl Snapshot {
    /// Snapshot
    pub fn new() -> Snapshot {
        Snapshot {
            id: None,
            name: None,
            created_at: None,
            state: None,
            size: None,
            export: None,
            instance: None,
        }
    }
}

/// Snapshot state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "snapshotting")]
    Snapshotting,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "exporting")]
    Exporting,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "exported")]
    Exported,
}

impl Default for State {
    fn default() -> State {
        Self::Snapshotting
    }
}
