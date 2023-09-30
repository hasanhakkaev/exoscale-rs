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
pub struct ScaleInstanceRequest {
    #[serde(rename = "instance-type")]
    pub instance_type: Box<crate::models::InstanceType>,
}

impl ScaleInstanceRequest {
    pub fn new(instance_type: crate::models::InstanceType) -> ScaleInstanceRequest {
        ScaleInstanceRequest {
            instance_type: Box::new(instance_type),
        }
    }
}


