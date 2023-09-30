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
pub struct RegisterTemplateRequest {
    /// Template maintainer
    #[serde(rename = "maintainer", skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,
    /// Template description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Enable SSH key-based login
    #[serde(rename = "ssh-key-enabled")]
    pub ssh_key_enabled: bool,
    /// Template name
    #[serde(rename = "name")]
    pub name: String,
    /// Template default user
    #[serde(rename = "default-user", skip_serializing_if = "Option::is_none")]
    pub default_user: Option<String>,
    /// Template size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Enable password-based login
    #[serde(rename = "password-enabled")]
    pub password_enabled: bool,
    /// Template build
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    /// Template MD5 checksum
    #[serde(rename = "checksum")]
    pub checksum: String,
    /// Boot mode (default: legacy)
    #[serde(rename = "boot-mode", skip_serializing_if = "Option::is_none")]
    pub boot_mode: Option<BootMode>,
    /// Template source URL
    #[serde(rename = "url")]
    pub url: String,
    /// Template version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl RegisterTemplateRequest {
    pub fn new(ssh_key_enabled: bool, name: String, password_enabled: bool, checksum: String, url: String) -> RegisterTemplateRequest {
        RegisterTemplateRequest {
            maintainer: None,
            description: None,
            ssh_key_enabled,
            name,
            default_user: None,
            size: None,
            password_enabled,
            build: None,
            checksum,
            boot_mode: None,
            url,
            version: None,
        }
    }
}

/// Boot mode (default: legacy)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BootMode {
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "uefi")]
    Uefi,
}

impl Default for BootMode {
    fn default() -> BootMode {
        Self::Legacy
    }
}
