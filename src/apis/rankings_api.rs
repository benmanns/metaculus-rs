/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`rankings_list`]
#[derive(Clone, Debug)]
pub struct RankingsListParams {
    /// Number of results to return per page.
    pub limit: Option<i32>,
    /// The initial index from which to return the results.
    pub offset: Option<i32>,
}

/// struct for passing parameters to the method [`rankings_retrieve`]
#[derive(Clone, Debug)]
pub struct RankingsRetrieveParams {
    /// A unique integer value identifying this site-specific user data.
    pub id: i32,
}

/// struct for typed errors of method [`rankings_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RankingsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rankings_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RankingsRetrieveError {
    UnknownValue(serde_json::Value),
}

pub async fn rankings_list(
    configuration: &configuration::Configuration,
    params: RankingsListParams,
) -> Result<crate::models::PaginatedRankingList, Error<RankingsListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let offset = params.offset;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/rankings/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_cookie) = local_var_configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header("Cookie", format!("sessionid={}", local_var_cookie.value));
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RankingsListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn rankings_retrieve(
    configuration: &configuration::Configuration,
    params: RankingsRetrieveParams,
) -> Result<crate::models::Ranking, Error<RankingsRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/rankings/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_cookie) = local_var_configuration.cookie {
        local_var_req_builder =
            local_var_req_builder.header("Cookie", format!("sessionid={}", local_var_cookie.value));
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RankingsRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
