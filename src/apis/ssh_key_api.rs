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


/// struct for typed errors of method [`delete_ssh_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteSshKeyError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`get_ssh_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSshKeyError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`list_ssh_keys`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSshKeysError {
    UnknownValue(serde_json::Value),
}


/// struct for typed errors of method [`register_ssh_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterSshKeyError {
    UnknownValue(serde_json::Value),
}


pub async fn delete_ssh_key(configuration: &configuration::Configuration, name: &str) -> Result<models::Operation, Error<DeleteSshKeyError>> {
    let local_var_name = name;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("name".to_string(), crate::apis::urlencode(local_var_name));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::DELETE,
    "/ssh-key/{name}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn get_ssh_key(configuration: &configuration::Configuration, name: &str) -> Result<models::SshKey, Error<GetSshKeyError>> {
    let local_var_name = name;

    let mut path_params_map = std::collections::HashMap::new();
                path_params_map.insert("name".to_string(), crate::apis::urlencode(local_var_name));

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/ssh-key/{name}",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn list_ssh_keys(configuration: &configuration::Configuration, ) -> Result<models::ListSshKeys200Response, Error<ListSshKeysError>> {

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
        let body_payload_option: Option<()> = None;

    utils::execute_request(
    configuration,
    reqwest::Method::GET,
    "/ssh-key",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
pub async fn register_ssh_key(configuration: &configuration::Configuration, register_ssh_key_request: models::RegisterSshKeyRequest) -> Result<models::Operation, Error<RegisterSshKeyError>> {
    let local_var_register_ssh_key_request = register_ssh_key_request;

    let path_params_map = std::collections::HashMap::new();

    let query_params_vec: Vec<(&str, String)> = Vec::new();
    let query_params_option = if query_params_vec.is_empty() { None } else { Some(query_params_vec.as_slice())};
            let body_payload_option = Some(local_var_register_ssh_key_request);

    utils::execute_request(
    configuration,
    reqwest::Method::POST,
    "/ssh-key",
    path_params_map,
    query_params_option,
    body_payload_option,
    ).await
}
