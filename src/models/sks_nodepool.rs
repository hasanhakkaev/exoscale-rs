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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SksNodepool {
    /// Nodepool Anti-affinity Groups
    #[serde(rename = "anti-affinity-groups", skip_serializing_if = "Option::is_none")]
    pub anti_affinity_groups: Option<Vec<models::AntiAffinityGroup>>,
    /// Nodepool description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nodepool public IP assignment of the Instances:  * IPv4 (`inet4`) addressing only; * IPv4 and IPv6 (`dual`) addressing.
    #[serde(rename = "public-ip-assignment", skip_serializing_if = "Option::is_none")]
    pub public_ip_assignment: Option<PublicIpAssignment>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "taints", skip_serializing_if = "Option::is_none")]
    pub taints: Option<std::collections::HashMap<String, models::SksNodepoolTaint>>,
    /// Nodepool Security Groups
    #[serde(rename = "security-groups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<models::SecurityGroup>>,
    /// Nodepool name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "instance-type", skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<Box<models::InstanceType>>,
    /// Nodepool Private Networks
    #[serde(rename = "private-networks", skip_serializing_if = "Option::is_none")]
    pub private_networks: Option<Vec<models::PrivateNetwork>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<models::Template>>,
    /// Nodepool state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Number of instances
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "kubelet-image-gc", skip_serializing_if = "Option::is_none")]
    pub kubelet_image_gc: Option<Box<models::KubeletImageGc>>,
    #[serde(rename = "instance-pool", skip_serializing_if = "Option::is_none")]
    pub instance_pool: Option<Box<models::InstancePool>>,
    /// The instances created by the Nodepool will be prefixed with this value (default: pool)
    #[serde(rename = "instance-prefix", skip_serializing_if = "Option::is_none")]
    pub instance_prefix: Option<String>,
    #[serde(rename = "deploy-target", skip_serializing_if = "Option::is_none")]
    pub deploy_target: Option<Box<models::DeployTarget>>,
    /// Nodepool addons
    #[serde(rename = "addons", skip_serializing_if = "Option::is_none")]
    pub addons: Option<std::collections::HashSet<Addons>>,
    /// Nodepool ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// Nodepool instances disk size in GiB
    #[serde(rename = "disk-size", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<u64>,
    /// Nodepool version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Nodepool creation date
    #[serde(rename = "created-at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl SksNodepool {
    /// SKS Nodepool
    pub fn new() -> SksNodepool {
        SksNodepool {
            anti_affinity_groups: None,
            description: None,
            public_ip_assignment: None,
            labels: None,
            taints: None,
            security_groups: None,
            name: None,
            instance_type: None,
            private_networks: None,
            template: None,
            state: None,
            size: None,
            kubelet_image_gc: None,
            instance_pool: None,
            instance_prefix: None,
            deploy_target: None,
            addons: None,
            id: None,
            disk_size: None,
            version: None,
            created_at: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PublicIpAssignment {
    #[serde(rename = "inet4")]
    Inet4,
    #[serde(rename = "dual")]
    Dual,
}

impl Default for PublicIpAssignment {
    fn default() -> PublicIpAssignment {
        Self::Inet4
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "renewing-token")]
    RenewingToken,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "deleting")]
    Deleting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "scaling")]
    Scaling,
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "error")]
    Error,
}

impl Default for State {
    fn default() -> State {
        Self::RenewingToken
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Addons {
    #[serde(rename = "storage-lvm")]
    StorageLvm,
}

impl Default for Addons {
    fn default() -> Addons {
        Self::StorageLvm
    }
}

