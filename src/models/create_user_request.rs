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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// User Email
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<models::IamRole>>,
}

impl CreateUserRequest {
    pub fn new(email: String) -> CreateUserRequest {
        CreateUserRequest { email, role: None }
    }
}
