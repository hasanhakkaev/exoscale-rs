/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SksOidc : SKS Cluster OpenID config map

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SksOidc {
    /// OpenID client ID
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// OpenID provider URL
    #[serde(rename = "issuer-url")]
    pub issuer_url: String,
    /// JWT claim to use as the user name
    #[serde(rename = "username-claim", skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
    /// Prefix prepended to username claims
    #[serde(rename = "username-prefix", skip_serializing_if = "Option::is_none")]
    pub username_prefix: Option<String>,
    /// JWT claim to use as the user's group
    #[serde(rename = "groups-claim", skip_serializing_if = "Option::is_none")]
    pub groups_claim: Option<String>,
    /// Prefix prepended to group claims
    #[serde(rename = "groups-prefix", skip_serializing_if = "Option::is_none")]
    pub groups_prefix: Option<String>,
    /// A key value map that describes a required claim in the ID Token
    #[serde(rename = "required-claim", skip_serializing_if = "Option::is_none")]
    pub required_claim: Option<::std::collections::HashMap<String, String>>,
}

impl SksOidc {
    /// SKS Cluster OpenID config map
    pub fn new(client_id: String, issuer_url: String) -> SksOidc {
        SksOidc {
            client_id,
            issuer_url,
            username_claim: None,
            username_prefix: None,
            groups_claim: None,
            groups_prefix: None,
            required_claim: None,
        }
    }
}