/*
 * Metaculus API
 *
 *  <b>Welcome to the official Metaculus API!</b>  If you have questions, ideas, or feedback, please contact our team at api-requests@metaculus.com. We are excited to keep building upon this initial version of the API, and we’d like to keep making it more useful to you. Our aim is to support the forecasting community – we’re listening!  <b>Get Started in 15 Seconds</b>  <ol> <li>Most of the API is (hopefully) self-explanatory. You’ll find all the documentation below. <li>If you’re testing the waters or doing a one-off analysis, you can dive right in! <li>If you’re building an application that connects to the Metaculus API, you’ll need to authenticate. </ol>  <b>How to Authenticate</b>  To request an auth token, email api-requests@metaculus.com and let Dan, Jon, and Martin know what you’d like to build. Please tell us a bit about yourself or your organization and how you intend to use the API.  You can then add the token we generate for you to your requests using the <i>Authorization</i> HTTP header. The token should be prefixed by the string  literal \"Token\", with whitespace separating the two strings.  Example:  <blockquote> Authorization: Token 9944b09199c62bcf9418ad846dd0e4bbdfc6ee4b </blockquote>  <b>A Note for Early Adopters of the API from 2020-2023</b>  Please be aware that we‘re rapidly improving our forecasting tools and so cannot guarantee continued backward compatibility for all features. If you previously authenticated via cookies, we will continue to support this functionality for now, but we ask that you please switch to using auth tokens as described above by June 1, 2023.  We're eager to support wider usage of the API, and the more feedback we get from the community, the more helpful we can be. (For certain endpoints, access levels vary depending on user permissions.)  Please send us an email at api-requests@metaculus.com if you’d like to discuss your specific application.  <i>Updated as of: May 1, 2023</i>  <hr/> <h3 id=\"tutorial\"> A Very Quick Tutorial</h3>  Let's walk through the process of fetching a list of questions, getting the details of an individual question, and making a prediction.  <b>Fetching a Question List and Question Details</b>  This is straightforward: we set the <i>Authorization</i> header as described above, and fetch the URL  <i>https://www.metaculus.com/api2/questions/</i>  Let's make it a little more interesting by only fetching questions which were asked during  and resolved before the end of, 2022. We can add the query parameters  <i>publish_time__gt</i> and <i>resolve_time__lt</i> accordingly  (note double underscores before <i>gt</i> and </i>let</i>):  <i>https://www.metaculus.com/api2/questions/?publish_time__gt=2021-12-31&resolve_time__lt=2023-01-01</i>  (We could make it more precise by saying e.g. <i>public_time__t=2021-12-31T23:59:59Z</i> but this is sufficient for our purposes.) This gets us the question with ID 7871, <i>\"Will there be a female Fields Medalist in 2022?\"</i>  In practice, the data from <i>/questions</i> includes almost all the data about a question -- and even more, such as the description and its categories, if you also add the query parameter <i>include_description=true</i>. But you could easily get the few additional details from  <i>https://www.metaculus.com/api2/questions/7871</i>  <b>Making a Prediction</b>  We can't <i>actually</i> post a new prediction for that question, since it has resolved. But if we were to time-travel back to before its resolution, we would do so by simply HTTP POSTing to the URL  <i>https://www.metaculus.com/api2/questions/7871/predict</i>  the payload  {     \"prediction\": 0.9, }  to predict a 90% chance of this happening. If we <i>did</i> have the permissions required to make that prediction, we would get back a response with the prediction we just made. Note that this is a binary (i.e. yes/no) question; for a <i>continuous</i> question, the payload gets a little more complicated.  <b>Making A Continuous Prediction</b>  ...OK: a <i>lot</i> more complicated. For now, we'll just document the JSON schema that such predictions require below, and will document its usage more thoroughly it in a separate subsequent tutorial. In the interim, please contact us at <a href=\"mailto:api-requests@metaculus.com\">api-requests@metaculus.com</a> if you wish to use the API to make continuous predictions.  <b>JSON Schema for Continuous Predictions</b>  <blockquote> schema = {     \"type\": \"object\",     \"properties\": {         \"kind\": {             \"enum\": ([\"logistic\", \"gaussian\"])         },         \"avg\": {             \"type\": \"number\",             \"minimum\": -2,             \"maximum\": 3,         },         \"stdev\": {             \"type\": \"number\",             \"minimum\": 0.005,             \"maximum\": 10,         },         \"a\": {             \"type\": \"number\",             \"minimum\": -0.96,             \"maximum\": +0.96,         },         \"low\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },         \"high\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },     },     \"additionalProperties\": boolean,     \"required\": [\"avg\", \"stdev\"] } </blockquote> For questions with lower/upper bounds, \"low\"/\"high\" are required. For logistic distributions, the schema requires \"x0\" and \"s\" rather than \"avg\" and \"stdev.\" Metaculus supports multiple superimposed distributions, in which case the above schema will be nested in a \"super-schema\" containing each of those; contact us for details. <hr/>
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`comments_create`]
#[derive(Clone, Debug)]
pub struct CommentsCreateParams {
    pub comment: crate::models::Comment,
}

/// struct for passing parameters to the method [`comments_destroy`]
#[derive(Clone, Debug)]
pub struct CommentsDestroyParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
}

/// struct for passing parameters to the method [`comments_like_create`]
#[derive(Clone, Debug)]
pub struct CommentsLikeCreateParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
    pub comment: crate::models::Comment,
}

/// struct for passing parameters to the method [`comments_list`]
#[derive(Clone, Debug)]
pub struct CommentsListParams {
    pub author: Option<i32>,
    pub created_time__gt: Option<String>,
    pub created_time__lt: Option<String>,
    /// The pagination cursor value.
    pub cursor: Option<String>,
    pub id: Option<i32>,
    /// Number of results to return per page.
    pub limit: Option<i32>,
    /// Which field to use when ordering the results.
    pub order_by: Option<String>,
    pub question: Option<i32>,
}

/// struct for passing parameters to the method [`comments_partial_update`]
#[derive(Clone, Debug)]
pub struct CommentsPartialUpdateParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
    pub patched_comment_update: Option<crate::models::PatchedCommentUpdate>,
}

/// struct for passing parameters to the method [`comments_report_create`]
#[derive(Clone, Debug)]
pub struct CommentsReportCreateParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
    pub comment: crate::models::Comment,
}

/// struct for passing parameters to the method [`comments_retrieve`]
#[derive(Clone, Debug)]
pub struct CommentsRetrieveParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
}

/// struct for passing parameters to the method [`comments_update`]
#[derive(Clone, Debug)]
pub struct CommentsUpdateParams {
    /// A unique integer value identifying this comment.
    pub id: i32,
    pub comment_update: crate::models::CommentUpdate,
}

/// struct for typed errors of method [`comments_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsDestroyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_like_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsLikeCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_partial_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsPartialUpdateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_report_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsReportCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`comments_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentsUpdateError {
    UnknownValue(serde_json::Value),
}

pub async fn comments_create(
    configuration: &configuration::Configuration,
    params: CommentsCreateParams,
) -> Result<crate::models::Comment, Error<CommentsCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let comment = params.comment;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/comments/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&comment);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CommentsCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_destroy(
    configuration: &configuration::Configuration,
    params: CommentsDestroyParams,
) -> Result<(), Error<CommentsDestroyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
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
        Ok(())
    } else {
        let local_var_entity: Option<CommentsDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_like_create(
    configuration: &configuration::Configuration,
    params: CommentsLikeCreateParams,
) -> Result<crate::models::Comment, Error<CommentsLikeCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let comment = params.comment;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/like/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&comment);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CommentsLikeCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_list(
    configuration: &configuration::Configuration,
    params: CommentsListParams,
) -> Result<crate::models::PaginatedCommentList, Error<CommentsListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let author = params.author;
    let created_time__gt = params.created_time__gt;
    let created_time__lt = params.created_time__lt;
    let cursor = params.cursor;
    let id = params.id;
    let limit = params.limit;
    let order_by = params.order_by;
    let question = params.question;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/comments/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = author {
        local_var_req_builder =
            local_var_req_builder.query(&[("author", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_time__gt {
        local_var_req_builder =
            local_var_req_builder.query(&[("created_time__gt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_time__lt {
        local_var_req_builder =
            local_var_req_builder.query(&[("created_time__lt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder =
            local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = id {
        local_var_req_builder = local_var_req_builder.query(&[("id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = question {
        local_var_req_builder =
            local_var_req_builder.query(&[("question", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
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
        let local_var_entity: Option<CommentsListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_partial_update(
    configuration: &configuration::Configuration,
    params: CommentsPartialUpdateParams,
) -> Result<crate::models::CommentUpdate, Error<CommentsPartialUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let patched_comment_update = params.patched_comment_update;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&patched_comment_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CommentsPartialUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_report_create(
    configuration: &configuration::Configuration,
    params: CommentsReportCreateParams,
) -> Result<crate::models::Comment, Error<CommentsReportCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let comment = params.comment;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/report/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&comment);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CommentsReportCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_retrieve(
    configuration: &configuration::Configuration,
    params: CommentsRetrieveParams,
) -> Result<crate::models::CommentChildren, Error<CommentsRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/",
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
        let local_var_entity: Option<CommentsRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn comments_update(
    configuration: &configuration::Configuration,
    params: CommentsUpdateParams,
) -> Result<crate::models::CommentUpdate, Error<CommentsUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let comment_update = params.comment_update;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/comments/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder
            .basic_auth(local_var_auth_conf.0.clone(), local_var_auth_conf.1.clone());
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{local_var_prefix} {local_var_key}"),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&comment_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CommentsUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
