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
pub struct UpdateSksNodepoolRequest {
    /// Nodepool Anti-affinity Groups
    #[serde(rename = "anti-affinity-groups", skip_serializing_if = "Option::is_none")]
    pub anti_affinity_groups: Option<Vec<crate::models::AntiAffinityGroup>>,
    /// Nodepool description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "taints", skip_serializing_if = "Option::is_none")]
    pub taints: Option<::std::collections::HashMap<String, crate::models::SksNodepoolTaint>>,
    /// Nodepool Security Groups
    #[serde(rename = "security-groups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<crate::models::SecurityGroup>>,
    /// Nodepool name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "instance-type", skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<Box<crate::models::InstanceType>>,
    /// Nodepool Private Networks
    #[serde(rename = "private-networks", skip_serializing_if = "Option::is_none")]
    pub private_networks: Option<Vec<crate::models::PrivateNetwork>>,
    /// Prefix to apply to managed instances names (default: pool)
    #[serde(rename = "instance-prefix", skip_serializing_if = "Option::is_none")]
    pub instance_prefix: Option<String>,
    #[serde(rename = "deploy-target", skip_serializing_if = "Option::is_none")]
    pub deploy_target: Option<Box<crate::models::DeployTarget>>,
    /// Nodepool instances disk size in GB
    #[serde(rename = "disk-size", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
}

impl UpdateSksNodepoolRequest {
    pub fn new() -> UpdateSksNodepoolRequest {
        UpdateSksNodepoolRequest {
            anti_affinity_groups: None,
            description: None,
            labels: None,
            taints: None,
            security_groups: None,
            name: None,
            instance_type: None,
            private_networks: None,
            instance_prefix: None,
            deploy_target: None,
            disk_size: None,
        }
    }
}


