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

/// struct for passing parameters to the method [`questions_add_consideration_vote_create`]
#[derive(Clone, Debug)]
pub struct QuestionsAddConsiderationVoteCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_boost_create`]
#[derive(Clone, Debug)]
pub struct QuestionsBoostCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub boost: crate::models::Boost,
}

/// struct for passing parameters to the method [`questions_bulk_predict_create`]
#[derive(Clone, Debug)]
pub struct QuestionsBulkPredictCreateParams {
    pub bulk_prediction_input: crate::models::BulkPredictionInput,
}

/// struct for passing parameters to the method [`questions_create`]
#[derive(Clone, Debug)]
pub struct QuestionsCreateParams {
    pub question_update: Option<crate::models::QuestionUpdate>,
}

/// struct for passing parameters to the method [`questions_destroy`]
#[derive(Clone, Debug)]
pub struct QuestionsDestroyParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_list`]
#[derive(Clone, Debug)]
pub struct QuestionsListParams {
    pub access: Option<String>,
    pub author: Option<i32>,
    pub categories: Option<String>,
    pub close_time__gt: Option<String>,
    pub close_time__lt: Option<String>,
    pub commented_by: Option<f32>,
    pub contest: Option<String>,
    pub forecast_type: Option<String>,
    pub group: Option<i32>,
    pub guessed_by: Option<f32>,
    pub has_group: Option<bool>,
    /// Set to 'true' to include the description (and categories) in responses
    pub include_description: Option<String>,
    /// Number of results to return per page.
    pub limit: Option<i32>,
    pub not_guessed_by: Option<f32>,
    /// The initial index from which to return the results.
    pub offset: Option<i32>,
    /// Which field to use when ordering the results.
    pub order_by: Option<String>,
    pub project: Option<String>,
    pub publish_time__gt: Option<String>,
    pub publish_time__lt: Option<String>,
    pub resolve_time__gt: Option<String>,
    pub resolve_time__lt: Option<String>,
    pub reversed_related: Option<f32>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub r#type: Option<String>,
    pub unconditional: Option<bool>,
    pub upvoted_by: Option<f32>,
    pub username: Option<String>,
    pub visible_from_project: Option<String>,
}

/// struct for passing parameters to the method [`questions_partial_update`]
#[derive(Clone, Debug)]
pub struct QuestionsPartialUpdateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub patched_question_update: Option<crate::models::PatchedQuestionUpdate>,
}

/// struct for passing parameters to the method [`questions_predict_create`]
#[derive(Clone, Debug)]
pub struct QuestionsPredictCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub prediction_input: crate::models::PredictionInput,
}

/// struct for passing parameters to the method [`questions_prediction_for_date_retrieve`]
#[derive(Clone, Debug)]
pub struct QuestionsPredictionForDateRetrieveParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_prediction_history_retrieve`]
#[derive(Clone, Debug)]
pub struct QuestionsPredictionHistoryRetrieveParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_predictions_retrieve`]
#[derive(Clone, Debug)]
pub struct QuestionsPredictionsRetrieveParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_question_sharing_create`]
#[derive(Clone, Debug)]
pub struct QuestionsQuestionSharingCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub username: String,
    pub share_question: crate::models::ShareQuestion,
}

/// struct for passing parameters to the method [`questions_question_sharing_destroy`]
#[derive(Clone, Debug)]
pub struct QuestionsQuestionSharingDestroyParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub username: String,
}

/// struct for passing parameters to the method [`questions_remove_consideration_vote_create`]
#[derive(Clone, Debug)]
pub struct QuestionsRemoveConsiderationVoteCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_resolve_create`]
#[derive(Clone, Debug)]
pub struct QuestionsResolveCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub question_resolve: crate::models::QuestionResolve,
}

/// struct for passing parameters to the method [`questions_retrieve`]
#[derive(Clone, Debug)]
pub struct QuestionsRetrieveParams {
    /// A unique integer value identifying this question.
    pub id: i32,
}

/// struct for passing parameters to the method [`questions_show_community_create`]
#[derive(Clone, Debug)]
pub struct QuestionsShowCommunityCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub show_community: crate::models::ShowCommunity,
}

/// struct for passing parameters to the method [`questions_update`]
#[derive(Clone, Debug)]
pub struct QuestionsUpdateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub question_update: Option<crate::models::QuestionUpdate>,
}

/// struct for passing parameters to the method [`questions_view_metaculus_prediction_create`]
#[derive(Clone, Debug)]
pub struct QuestionsViewMetaculusPredictionCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub question: crate::models::Question,
}

/// struct for passing parameters to the method [`questions_vote_create`]
#[derive(Clone, Debug)]
pub struct QuestionsVoteCreateParams {
    /// A unique integer value identifying this question.
    pub id: i32,
    pub question_vote: crate::models::QuestionVote,
}

/// struct for typed errors of method [`questions_add_consideration_vote_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsAddConsiderationVoteCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_boost_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsBoostCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_bulk_predict_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsBulkPredictCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsDestroyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_partial_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsPartialUpdateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_predict_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsPredictCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_prediction_for_date_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsPredictionForDateRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_prediction_history_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsPredictionHistoryRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_predictions_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsPredictionsRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_question_sharing_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsQuestionSharingCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_question_sharing_destroy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsQuestionSharingDestroyError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_remove_consideration_vote_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsRemoveConsiderationVoteCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_resolve_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsResolveCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_show_community_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsShowCommunityCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsUpdateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_view_metaculus_prediction_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsViewMetaculusPredictionCreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`questions_vote_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuestionsVoteCreateError {
    UnknownValue(serde_json::Value),
}

pub async fn questions_add_consideration_vote_create(
    configuration: &configuration::Configuration,
    params: QuestionsAddConsiderationVoteCreateParams,
) -> Result<(), Error<QuestionsAddConsiderationVoteCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/add_consideration_vote/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsAddConsiderationVoteCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_boost_create(
    configuration: &configuration::Configuration,
    params: QuestionsBoostCreateParams,
) -> Result<crate::models::Boost, Error<QuestionsBoostCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let boost = params.boost;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/boost/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&boost);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsBoostCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_bulk_predict_create(
    configuration: &configuration::Configuration,
    params: QuestionsBulkPredictCreateParams,
) -> Result<crate::models::BulkPredictionInput, Error<QuestionsBulkPredictCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bulk_prediction_input = params.bulk_prediction_input;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/bulk-predict/",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&bulk_prediction_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsBulkPredictCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_create(
    configuration: &configuration::Configuration,
    params: QuestionsCreateParams,
) -> Result<crate::models::QuestionUpdate, Error<QuestionsCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let question_update = params.question_update;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/questions/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&question_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_destroy(
    configuration: &configuration::Configuration,
    params: QuestionsDestroyParams,
) -> Result<(), Error<QuestionsDestroyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_list(
    configuration: &configuration::Configuration,
    params: QuestionsListParams,
) -> Result<crate::models::PaginatedQuestionUserList, Error<QuestionsListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let access = params.access;
    let author = params.author;
    let categories = params.categories;
    let close_time__gt = params.close_time__gt;
    let close_time__lt = params.close_time__lt;
    let commented_by = params.commented_by;
    let contest = params.contest;
    let forecast_type = params.forecast_type;
    let group = params.group;
    let guessed_by = params.guessed_by;
    let has_group = params.has_group;
    let include_description = params.include_description;
    let limit = params.limit;
    let not_guessed_by = params.not_guessed_by;
    let offset = params.offset;
    let order_by = params.order_by;
    let project = params.project;
    let publish_time__gt = params.publish_time__gt;
    let publish_time__lt = params.publish_time__lt;
    let resolve_time__gt = params.resolve_time__gt;
    let resolve_time__lt = params.resolve_time__lt;
    let reversed_related = params.reversed_related;
    let search = params.search;
    let status = params.status;
    let r#type = params.r#type;
    let unconditional = params.unconditional;
    let upvoted_by = params.upvoted_by;
    let username = params.username;
    let visible_from_project = params.visible_from_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/questions/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = access {
        local_var_req_builder =
            local_var_req_builder.query(&[("access", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = author {
        local_var_req_builder =
            local_var_req_builder.query(&[("author", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = categories {
        local_var_req_builder =
            local_var_req_builder.query(&[("categories", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = close_time__gt {
        local_var_req_builder =
            local_var_req_builder.query(&[("close_time__gt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = close_time__lt {
        local_var_req_builder =
            local_var_req_builder.query(&[("close_time__lt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = commented_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("commented_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = contest {
        local_var_req_builder =
            local_var_req_builder.query(&[("contest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = forecast_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("forecast_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = group {
        local_var_req_builder =
            local_var_req_builder.query(&[("group", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = guessed_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("guessed_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = has_group {
        local_var_req_builder =
            local_var_req_builder.query(&[("has_group", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include_description {
        local_var_req_builder =
            local_var_req_builder.query(&[("include_description", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = not_guessed_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("not_guessed_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder =
            local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = project {
        local_var_req_builder =
            local_var_req_builder.query(&[("project", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = publish_time__gt {
        local_var_req_builder =
            local_var_req_builder.query(&[("publish_time__gt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = publish_time__lt {
        local_var_req_builder =
            local_var_req_builder.query(&[("publish_time__lt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resolve_time__gt {
        local_var_req_builder =
            local_var_req_builder.query(&[("resolve_time__gt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resolve_time__lt {
        local_var_req_builder =
            local_var_req_builder.query(&[("resolve_time__lt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = reversed_related {
        local_var_req_builder =
            local_var_req_builder.query(&[("reversed_related", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = unconditional {
        local_var_req_builder =
            local_var_req_builder.query(&[("unconditional", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upvoted_by {
        local_var_req_builder =
            local_var_req_builder.query(&[("upvoted_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username {
        local_var_req_builder =
            local_var_req_builder.query(&[("username", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = visible_from_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("visible_from_project", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_partial_update(
    configuration: &configuration::Configuration,
    params: QuestionsPartialUpdateParams,
) -> Result<crate::models::QuestionUpdate, Error<QuestionsPartialUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let patched_question_update = params.patched_question_update;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&patched_question_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsPartialUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_predict_create(
    configuration: &configuration::Configuration,
    params: QuestionsPredictCreateParams,
) -> Result<crate::models::PredictionInput, Error<QuestionsPredictCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let prediction_input = params.prediction_input;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/predict/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&prediction_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsPredictCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used only for Tezos verification
pub async fn questions_prediction_for_date_retrieve(
    configuration: &configuration::Configuration,
    params: QuestionsPredictionForDateRetrieveParams,
) -> Result<crate::models::PredictionForDate, Error<QuestionsPredictionForDateRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/prediction-for-date/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsPredictionForDateRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_prediction_history_retrieve(
    configuration: &configuration::Configuration,
    params: QuestionsPredictionHistoryRetrieveParams,
) -> Result<(), Error<QuestionsPredictionHistoryRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/prediction-history/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsPredictionHistoryRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_predictions_retrieve(
    configuration: &configuration::Configuration,
    params: QuestionsPredictionsRetrieveParams,
) -> Result<(), Error<QuestionsPredictionsRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/predictions/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsPredictionsRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used for sharing Private and Draft questions with other users. If sharing private question - invited user will receive \"predictor\" permissions. If sharing draft question - invited user will receive \"coauthor\" permissions\"
pub async fn questions_question_sharing_create(
    configuration: &configuration::Configuration,
    params: QuestionsQuestionSharingCreateParams,
) -> Result<crate::models::ShareQuestion, Error<QuestionsQuestionSharingCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let username = params.username;
    let share_question = params.share_question;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/question-sharing/{username}/",
        local_var_configuration.base_path,
        id = id,
        username = crate::apis::urlencode(username)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&share_question);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsQuestionSharingCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used for sharing Private and Draft questions with other users. If sharing private question - invited user will receive \"predictor\" permissions. If sharing draft question - invited user will receive \"coauthor\" permissions\"
pub async fn questions_question_sharing_destroy(
    configuration: &configuration::Configuration,
    params: QuestionsQuestionSharingDestroyParams,
) -> Result<(), Error<QuestionsQuestionSharingDestroyError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let username = params.username;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/question-sharing/{username}/",
        local_var_configuration.base_path,
        id = id,
        username = crate::apis::urlencode(username)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsQuestionSharingDestroyError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_remove_consideration_vote_create(
    configuration: &configuration::Configuration,
    params: QuestionsRemoveConsiderationVoteCreateParams,
) -> Result<(), Error<QuestionsRemoveConsiderationVoteCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/remove_consideration_vote/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsRemoveConsiderationVoteCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_resolve_create(
    configuration: &configuration::Configuration,
    params: QuestionsResolveCreateParams,
) -> Result<crate::models::QuestionResolve, Error<QuestionsResolveCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let question_resolve = params.question_resolve;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/resolve/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&question_resolve);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsResolveCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_retrieve(
    configuration: &configuration::Configuration,
    params: QuestionsRetrieveParams,
) -> Result<crate::models::QuestionUserDetail, Error<QuestionsRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
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
        let local_var_entity: Option<QuestionsRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_show_community_create(
    configuration: &configuration::Configuration,
    params: QuestionsShowCommunityCreateParams,
) -> Result<crate::models::ShowCommunity, Error<QuestionsShowCommunityCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let show_community = params.show_community;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/show-community/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&show_community);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsShowCommunityCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_update(
    configuration: &configuration::Configuration,
    params: QuestionsUpdateParams,
) -> Result<crate::models::QuestionUpdate, Error<QuestionsUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let question_update = params.question_update;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&question_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_view_metaculus_prediction_create(
    configuration: &configuration::Configuration,
    params: QuestionsViewMetaculusPredictionCreateParams,
) -> Result<crate::models::Question, Error<QuestionsViewMetaculusPredictionCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let question = params.question;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/view-metaculus-prediction/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&question);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsViewMetaculusPredictionCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn questions_vote_create(
    configuration: &configuration::Configuration,
    params: QuestionsVoteCreateParams,
) -> Result<crate::models::QuestionVote, Error<QuestionsVoteCreateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let question_vote = params.question_vote;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/questions/{id}/vote/",
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
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&question_vote);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuestionsVoteCreateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}