/*
 * Exoscale Public API
 *
 *  Infrastructure automation API, allowing programmatic access to all Exoscale products and services.  The [OpenAPI Specification](http://spec.openapis.org/oas/v3.0.3.html) source of this documentation can be obtained here:  * [JSON format](https://openapi-v2.exoscale.com/source.json) * [YAML format](https://openapi-v2.exoscale.com/source.yaml)
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: api@exoscale.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use serde::{Deserialize, Serialize};
use crate::{models, utils,apis::ResponseContent};
use super::{Error, configuration};


/// struct for typed errors of method [`add_service_to_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddServiceToLoadBalancerError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`create_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLoadBalancerError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`delete_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLoadBalancerError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`delete_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`get_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoadBalancerError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`get_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`list_load_balancers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLoadBalancersError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`reset_load_balancer_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetLoadBalancerFieldError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`reset_load_balancer_service_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetLoadBalancerServiceFieldError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`update_load_balancer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLoadBalancerError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`update_load_balancer_service`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLoadBalancerServiceError {
    UnknownValue(serde_json::Value),
}


pub async fn add_service_to_load_balancer(configuration: &configuration::Configuration, id: String, add_service_to_load_balancer_request: models::AddServiceToLoadBalancerRequest) -> Result<models::Operation, Error<AddServiceToLoadBalancerError>> {
    let local_var_id = id;
    let local_var_add_service_to_load_balancer_request = add_service_to_load_balancer_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_add_service_to_load_balancer_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/load-balancer/{id}/service",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn create_load_balancer(configuration: &configuration::Configuration, create_load_balancer_request: models::CreateLoadBalancerRequest) -> Result<models::Operation, Error<CreateLoadBalancerError>> {
    let local_var_create_load_balancer_request = create_load_balancer_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_create_load_balancer_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/load-balancer",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_load_balancer(configuration: &configuration::Configuration, id: String) -> Result<models::Operation, Error<DeleteLoadBalancerError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/load-balancer/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_load_balancer_service(configuration: &configuration::Configuration, id: String, service_id: String) -> Result<models::Operation, Error<DeleteLoadBalancerServiceError>> {
    let local_var_id = id;
    let local_var_service_id = service_id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));
                path_params_map.insert("service_id".to_string(), crate::apis::urlencode(local_var_service_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/load-balancer/{id}/service/{service_id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_load_balancer(configuration: &configuration::Configuration, id: String) -> Result<models::LoadBalancer, Error<GetLoadBalancerError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/load-balancer/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_load_balancer_service(configuration: &configuration::Configuration, id: String, service_id: String) -> Result<models::LoadBalancerService, Error<GetLoadBalancerServiceError>> {
    let local_var_id = id;
    let local_var_service_id = service_id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));
                path_params_map.insert("service_id".to_string(), crate::apis::urlencode(local_var_service_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/load-balancer/{id}/service/{service_id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_load_balancers(configuration: &configuration::Configuration, ) -> Result<models::ListLoadBalancers200Response, Error<ListLoadBalancersError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/load-balancer",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn reset_load_balancer_field(configuration: &configuration::Configuration, id: String, field: &str) -> Result<models::Operation, Error<ResetLoadBalancerFieldError>> {
    let local_var_id = id;
    let local_var_field = field;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));
                path_params_map.insert("field".to_string(), crate::apis::urlencode(local_var_field));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/load-balancer/{id}/{field}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn reset_load_balancer_service_field(configuration: &configuration::Configuration, id: String, service_id: String, field: &str) -> Result<models::Operation, Error<ResetLoadBalancerServiceFieldError>> {
    let local_var_id = id;
    let local_var_service_id = service_id;
    let local_var_field = field;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));
                path_params_map.insert("service_id".to_string(), crate::apis::urlencode(local_var_service_id));
                path_params_map.insert("field".to_string(), crate::apis::urlencode(local_var_field));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/load-balancer/{id}/service/{service_id}/{field}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_load_balancer(configuration: &configuration::Configuration, id: String, update_load_balancer_request: models::UpdateLoadBalancerRequest) -> Result<models::Operation, Error<UpdateLoadBalancerError>> {
    let local_var_id = id;
    let local_var_update_load_balancer_request = update_load_balancer_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_load_balancer_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/load-balancer/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_load_balancer_service(configuration: &configuration::Configuration, id: String, service_id: String, update_load_balancer_service_request: models::UpdateLoadBalancerServiceRequest) -> Result<models::Operation, Error<UpdateLoadBalancerServiceError>> {
    let local_var_id = id;
    let local_var_service_id = service_id;
    let local_var_update_load_balancer_service_request = update_load_balancer_service_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));
                path_params_map.insert("service_id".to_string(), crate::apis::urlencode(local_var_service_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_load_balancer_service_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/load-balancer/{id}/service/{service_id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
