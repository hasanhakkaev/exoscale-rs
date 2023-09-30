/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SksCluster : SKS Cluster



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SksCluster {
    /// Cluster description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Cluster CNI
    #[serde(rename = "cni", skip_serializing_if = "Option::is_none")]
    pub cni: Option<Cni>,
    /// Enable auto upgrade of the control plane to the latest patch version available
    #[serde(rename = "auto-upgrade", skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    /// Cluster name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Cluster state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Cluster Nodepools
    #[serde(rename = "nodepools", skip_serializing_if = "Option::is_none")]
    pub nodepools: Option<Vec<crate::models::SksNodepool>>,
    /// Cluster level
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    /// Cluster addons
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<std::collections::HashSet<Addons>>,
    /// Cluster ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Control plane Kubernetes version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Cluster creation date
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Cluster endpoint
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

impl SksCluster {
    /// SKS Cluster
    pub fn new() -> SksCluster {
        SksCluster {
            description: None,
            labels: None,
            cni: None,
            auto_upgrade: None,
            name: None,
            state: None,
            nodepools: None,
            level: None,
            addons: None,
            id: None,
            version: None,
            created_at: None,
            endpoint: None,
        }
    }
}

/// Cluster CNI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cni {
    #[serde(rename = "calico")]
    Calico,
    #[serde(rename = "cilium")]
    Cilium,
}

impl Default for Cni {
    fn default() -> Cni {
        Self::Calico
    }
}
/// Cluster state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "rotating-ccm-credentials")]
    RotatingCcmCredentials,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "upgrading")]
    Upgrading,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "suspending")]
    Suspending,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "error")]
    Error,
}

impl Default for State {
    fn default() -> State {
        Self::RotatingCcmCredentials
    }
}
/// Cluster level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "starter")]
    Starter,
    #[serde(rename = "pro")]
    Pro,
}

impl Default for Level {
    fn default() -> Level {
        Self::Starter
    }
}
/// Cluster addons
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Addons {
    #[serde(rename = "exoscale-cloud-controller")]
    ExoscaleCloudController,
    #[serde(rename = "metrics-server")]
    MetricsServer,
}

impl Default for Addons {
    fn default() -> Addons {
        Self::ExoscaleCloudController
    }
}

