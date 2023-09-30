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
pub struct UpdateDbaasServiceGrafanaRequest {
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<crate::models::UpdateDbaasServiceMysqlRequestMaintenance>>,
    /// Subscription plan
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    /// Service is protected against termination and powering off
    #[serde(rename = "termination-protection", skip_serializing_if = "Option::is_none")]
    pub termination_protection: Option<bool>,
    /// Grafana specific settings
    #[serde(rename = "grafana-settings", skip_serializing_if = "Option::is_none")]
    pub grafana_settings: Option<serde_json::Value>,
    /// Allowed CIDR address blocks for incoming connections
    #[serde(rename = "ip-filter", skip_serializing_if = "Option::is_none")]
    pub ip_filter: Option<Vec<String>>,
}

impl UpdateDbaasServiceGrafanaRequest {
    pub fn new() -> UpdateDbaasServiceGrafanaRequest {
        UpdateDbaasServiceGrafanaRequest {
            maintenance: None,
            plan: None,
            termination_protection: None,
            grafana_settings: None,
            ip_filter: None,
        }
    }
}

