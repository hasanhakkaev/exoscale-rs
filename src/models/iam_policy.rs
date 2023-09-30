/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

/// IamPolicy : Policy



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IamPolicy {
    /// IAM default service strategy
    #[serde(rename = "default-service-strategy")]
    pub default_service_strategy: DefaultServiceStrategy,
    /// IAM services
    #[serde(rename = "services")]
    pub services: ::std::collections::HashMap<String, crate::models::IamServicePolicy>,
}

impl IamPolicy {
    /// Policy
    pub fn new(default_service_strategy: DefaultServiceStrategy, services: ::std::collections::HashMap<String, crate::models::IamServicePolicy>) -> IamPolicy {
        IamPolicy {
            default_service_strategy,
            services,
        }
    }
}

/// IAM default service strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultServiceStrategy {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

impl Default for DefaultServiceStrategy {
    fn default() -> DefaultServiceStrategy {
        Self::Allow
    }
}
