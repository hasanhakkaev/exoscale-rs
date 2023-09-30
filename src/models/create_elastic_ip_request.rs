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
pub struct CreateElasticIpRequest {
    /// Elastic IP address family (default: :inet4)
    #[serde(rename = "addressfamily", skip_serializing_if = "Option::is_none")]
    pub addressfamily: Option<Addressfamily>,
    /// Elastic IP description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<crate::models::ElasticIpHealthcheck>>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl CreateElasticIpRequest {
    pub fn new() -> CreateElasticIpRequest {
        CreateElasticIpRequest {
            addressfamily: None,
            description: None,
            healthcheck: None,
            labels: None,
        }
    }
}

/// Elastic IP address family (default: :inet4)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Addressfamily {
    #[serde(rename = "inet4")]
    Inet4,
    #[serde(rename = "inet6")]
    Inet6,
}

impl Default for Addressfamily {
    fn default() -> Addressfamily {
        Self::Inet4
    }
}

