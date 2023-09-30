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
pub struct UpdateInstancePoolRequest {
    /// Instance Pool Anti-affinity Groups
    #[serde(rename = "anti-affinity-groups", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub anti_affinity_groups: Option<Option<Vec<crate::models::AntiAffinityGroup>>>,
    /// Instance Pool description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "public-ip-assignment", skip_serializing_if = "Option::is_none")]
    pub public_ip_assignment: Option<crate::models::PublicIpAssignment>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Instance Pool Security Groups
    #[serde(rename = "security-groups", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Option<Vec<crate::models::SecurityGroup>>>,
    /// Instances Elastic IPs
    #[serde(rename = "elastic-ips", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<Option<Vec<crate::models::ElasticIp>>>,
    /// Instance Pool name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "instance-type", skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<Box<crate::models::InstanceType>>,
    /// Minimum number of running Instances
    #[serde(rename = "min-available", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub min_available: Option<Option<i64>>,
    /// Instance Pool Private Networks
    #[serde(rename = "private-networks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub private_networks: Option<Option<Vec<crate::models::PrivateNetwork>>>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<Box<crate::models::Template>>,
    #[serde(rename = "ssh-key", skip_serializing_if = "Option::is_none")]
    pub ssh_key: Option<Box<crate::models::SshKey>>,
    /// Prefix to apply to Instances names (default: pool)
    #[serde(rename = "instance-prefix", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance_prefix: Option<Option<String>>,
    /// Instances Cloud-init user-data
    #[serde(rename = "user-data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Option<String>>,
    #[serde(rename = "deploy-target", skip_serializing_if = "Option::is_none")]
    pub deploy_target: Option<Box<crate::models::DeployTarget>>,
    /// Enable IPv6. DEPRECATED: use `public-ip-assignments`.
    #[serde(rename = "ipv6-enabled", skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
    /// Instances disk size in GB
    #[serde(rename = "disk-size", skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    /// Instances SSH keys
    #[serde(rename = "ssh-keys", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Option<Vec<crate::models::SshKey>>>,
}

impl UpdateInstancePoolRequest {
    pub fn new() -> UpdateInstancePoolRequest {
        UpdateInstancePoolRequest {
            anti_affinity_groups: None,
            description: None,
            public_ip_assignment: None,
            labels: None,
            security_groups: None,
            elastic_ips: None,
            name: None,
            instance_type: None,
            min_available: None,
            private_networks: None,
            template: None,
            ssh_key: None,
            instance_prefix: None,
            user_data: None,
            deploy_target: None,
            ipv6_enabled: None,
            disk_size: None,
            ssh_keys: None,
        }
    }
}


