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
pub struct UpdateIamRoleRequest {
    /// IAM Role description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IAM Role permissions
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<std::collections::HashSet<Permissions>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl UpdateIamRoleRequest {
    pub fn new() -> UpdateIamRoleRequest {
        UpdateIamRoleRequest {
            description: None,
            permissions: None,
            labels: None,
        }
    }
}

/// IAM Role permissions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "bypass-governance-retention")]
    BypassGovernanceRetention,
}

impl Default for Permissions {
    fn default() -> Permissions {
        Self::BypassGovernanceRetention
    }
}

