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


/// struct for typed errors of method [`attach_instance_to_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttachInstanceToPrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`create_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatePrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`delete_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeletePrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`detach_instance_from_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetachInstanceFromPrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`get_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`list_private_networks`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListPrivateNetworksError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`reset_private_network_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetPrivateNetworkFieldError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`update_private_network`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePrivateNetworkError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`update_private_network_instance_ip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePrivateNetworkInstanceIpError {
    UnknownValue(serde_json::Value),
}


pub async fn attach_instance_to_private_network(configuration: &configuration::Configuration, id: String, attach_instance_to_private_network_request: models::AttachInstanceToPrivateNetworkRequest) -> Result<models::Operation, Error<AttachInstanceToPrivateNetworkError>> {
    let local_var_id = id;
    let local_var_attach_instance_to_private_network_request = attach_instance_to_private_network_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_attach_instance_to_private_network_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/private-network/{id}:attach",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn create_private_network(configuration: &configuration::Configuration, create_private_network_request: models::CreatePrivateNetworkRequest) -> Result<models::Operation, Error<CreatePrivateNetworkError>> {
    let local_var_create_private_network_request = create_private_network_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_create_private_network_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/private-network",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn delete_private_network(configuration: &configuration::Configuration, id: String) -> Result<models::Operation, Error<DeletePrivateNetworkError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/private-network/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn detach_instance_from_private_network(configuration: &configuration::Configuration, id: String, detach_instance_from_private_network_request: models::DetachInstanceFromPrivateNetworkRequest) -> Result<models::Operation, Error<DetachInstanceFromPrivateNetworkError>> {
    let local_var_id = id;
    let local_var_detach_instance_from_private_network_request = detach_instance_from_private_network_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_detach_instance_from_private_network_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/private-network/{id}:detach",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_private_network(configuration: &configuration::Configuration, id: String) -> Result<models::PrivateNetwork, Error<GetPrivateNetworkError>> {
    let local_var_id = id;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/private-network/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_private_networks(configuration: &configuration::Configuration, ) -> Result<models::ListPrivateNetworks200Response, Error<ListPrivateNetworksError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/private-network",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn reset_private_network_field(configuration: &configuration::Configuration, id: String, field: &str) -> Result<models::Operation, Error<ResetPrivateNetworkFieldError>> {
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
    "/private-network/{id}/{field}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_private_network(configuration: &configuration::Configuration, id: String, update_private_network_request: models::UpdatePrivateNetworkRequest) -> Result<models::Operation, Error<UpdatePrivateNetworkError>> {
    let local_var_id = id;
    let local_var_update_private_network_request = update_private_network_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_private_network_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/private-network/{id}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn update_private_network_instance_ip(configuration: &configuration::Configuration, id: String, update_private_network_instance_ip_request: models::UpdatePrivateNetworkInstanceIpRequest) -> Result<models::Operation, Error<UpdatePrivateNetworkInstanceIpError>> {
    let local_var_id = id;
    let local_var_update_private_network_instance_ip_request = update_private_network_instance_ip_request;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("id".to_string(), crate::apis::urlencode(local_var_id));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_update_private_network_instance_ip_request);

    utils::execute_request(
    configuration,
    reqwest::Method::PUT,
    "/private-network/{id}:update-ip",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
