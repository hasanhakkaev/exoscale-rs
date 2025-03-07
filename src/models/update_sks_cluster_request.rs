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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSksClusterRequest {
    /// Cluster name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Cluster description
    #[serde(
        rename = "description",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<Option<String>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "oidc", skip_serializing_if = "Option::is_none")]
    pub oidc: Option<Box<models::SksOidc>>,
    /// Enable auto upgrade of the control plane to the latest patch version available
    #[serde(rename = "auto-upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// Cluster addons
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<std::collections::HashSet<Addons>>,
}

impl UpdateSksClusterRequest {
    pub fn new() -> UpdateSksClusterRequest {
        UpdateSksClusterRequest {
            name: None,
            description: None,
            labels: None,
            oidc: None,
            auto_upgrade: None,
            addons: None,
        }
    }
}
/// Cluster addons
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Addons {
    #[serde(rename = "exoscale-cloud-controller")]
    ExoscaleCloudController,
    #[serde(rename = "exoscale-container-storage-interface")]
    ExoscaleContainerStorageInterface,
    #[serde(rename = "metrics-server")]
    MetricsServer,
}

impl Default for Addons {
    fn default() -> Addons {
        Self::ExoscaleCloudController
    }
}
