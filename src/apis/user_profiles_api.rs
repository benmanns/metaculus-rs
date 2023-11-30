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

/// struct for passing parameters to the method [`user_profiles_list`]
#[derive(Clone, Debug)]
pub struct UserProfilesListParams {
    /// A page number within the paginated result set.
    pub page: Option<i32>,
}

/// struct for passing parameters to the method [`user_profiles_partial_update`]
#[derive(Clone, Debug)]
pub struct UserProfilesPartialUpdateParams {
    pub first_name: String,
    pub id: i32,
    pub last_name: String,
    pub ask_when_reaffirm_question_modal: Option<bool>,
    pub date_joined: Option<String>,
    pub default_community_visibility: Option<i32>,
    pub default_mp_visibility: Option<i32>,
    pub email: Option<String>,
    pub formerly_known_as: Option<String>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub last_visited: Option<String>,
    pub level: Option<i32>,
    pub level_title: Option<String>,
    pub permissions: Option<::std::collections::HashMap<String, bool>>,
    pub powers: Option<::std::collections::HashMap<String, serde_json::Value>>,
    pub purchasable_track_record: Option<bool>,
    pub score: Option<i32>,
    pub show_profile_comments: Option<bool>,
    pub supporter_level: Option<i32>,
    pub supporter_since: Option<String>,
    pub tachyons: Option<i32>,
    pub url: Option<String>,
    pub username: Option<String>,
    pub username_change_cost: Option<i32>,
    pub patched_user_profile: Option<crate::models::PatchedUserProfile>,
}

/// struct for passing parameters to the method [`user_profiles_retrieve`]
#[derive(Clone, Debug)]
pub struct UserProfilesRetrieveParams {
    pub id: i32,
}

/// struct for passing parameters to the method [`user_profiles_update`]
#[derive(Clone, Debug)]
pub struct UserProfilesUpdateParams {
    pub first_name: String,
    pub id: i32,
    pub last_name: String,
    pub ask_when_reaffirm_question_modal: Option<bool>,
    pub date_joined: Option<String>,
    pub default_community_visibility: Option<i32>,
    pub default_mp_visibility: Option<i32>,
    pub email: Option<String>,
    pub formerly_known_as: Option<String>,
    pub is_staff: Option<bool>,
    pub is_superuser: Option<bool>,
    pub last_visited: Option<String>,
    pub level: Option<i32>,
    pub level_title: Option<String>,
    pub permissions: Option<::std::collections::HashMap<String, bool>>,
    pub powers: Option<::std::collections::HashMap<String, serde_json::Value>>,
    pub purchasable_track_record: Option<bool>,
    pub score: Option<i32>,
    pub show_profile_comments: Option<bool>,
    pub supporter_level: Option<i32>,
    pub supporter_since: Option<String>,
    pub tachyons: Option<i32>,
    pub url: Option<String>,
    pub username: Option<String>,
    pub username_change_cost: Option<i32>,
    pub user_profile: Option<crate::models::UserProfile>,
}

/// struct for typed errors of method [`user_profiles_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserProfilesListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_profiles_partial_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserProfilesPartialUpdateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_profiles_retrieve`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserProfilesRetrieveError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`user_profiles_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserProfilesUpdateError {
    UnknownValue(serde_json::Value),
}

pub async fn user_profiles_list(
    configuration: &configuration::Configuration,
    params: UserProfilesListParams,
) -> Result<crate::models::PaginatedUserProfileList, Error<UserProfilesListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page = params.page;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api2/user-profiles/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
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
        let local_var_entity: Option<UserProfilesListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_profiles_partial_update(
    configuration: &configuration::Configuration,
    params: UserProfilesPartialUpdateParams,
) -> Result<crate::models::UserProfile, Error<UserProfilesPartialUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let first_name = params.first_name;
    let id = params.id;
    let last_name = params.last_name;
    let ask_when_reaffirm_question_modal = params.ask_when_reaffirm_question_modal;
    let date_joined = params.date_joined;
    let default_community_visibility = params.default_community_visibility;
    let default_mp_visibility = params.default_mp_visibility;
    let email = params.email;
    let formerly_known_as = params.formerly_known_as;
    let is_staff = params.is_staff;
    let is_superuser = params.is_superuser;
    let last_visited = params.last_visited;
    let level = params.level;
    let level_title = params.level_title;
    let permissions = params.permissions;
    let powers = params.powers;
    let purchasable_track_record = params.purchasable_track_record;
    let score = params.score;
    let show_profile_comments = params.show_profile_comments;
    let supporter_level = params.supporter_level;
    let supporter_since = params.supporter_since;
    let tachyons = params.tachyons;
    let url = params.url;
    let username = params.username;
    let username_change_cost = params.username_change_cost;
    let patched_user_profile = params.patched_user_profile;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/user-profiles/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ask_when_reaffirm_question_modal {
        local_var_req_builder = local_var_req_builder.query(&[(
            "ask_when_reaffirm_question_modal",
            &local_var_str.to_string(),
        )]);
    }
    if let Some(ref local_var_str) = date_joined {
        local_var_req_builder =
            local_var_req_builder.query(&[("date_joined", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = default_community_visibility {
        local_var_req_builder = local_var_req_builder
            .query(&[("default_community_visibility", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = default_mp_visibility {
        local_var_req_builder =
            local_var_req_builder.query(&[("default_mp_visibility", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = email {
        local_var_req_builder =
            local_var_req_builder.query(&[("email", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("first_name", &first_name.to_string())]);
    if let Some(ref local_var_str) = formerly_known_as {
        local_var_req_builder =
            local_var_req_builder.query(&[("formerly_known_as", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_staff {
        local_var_req_builder =
            local_var_req_builder.query(&[("is_staff", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_superuser {
        local_var_req_builder =
            local_var_req_builder.query(&[("is_superuser", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("last_name", &last_name.to_string())]);
    if let Some(ref local_var_str) = last_visited {
        local_var_req_builder =
            local_var_req_builder.query(&[("last_visited", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = level {
        local_var_req_builder =
            local_var_req_builder.query(&[("level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = level_title {
        local_var_req_builder =
            local_var_req_builder.query(&[("levelTitle", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = permissions {
        local_var_req_builder =
            local_var_req_builder.query(&[("permissions", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = powers {
        local_var_req_builder =
            local_var_req_builder.query(&[("powers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = purchasable_track_record {
        local_var_req_builder = local_var_req_builder
            .query(&[("purchasable_track_record", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = score {
        local_var_req_builder =
            local_var_req_builder.query(&[("score", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_profile_comments {
        local_var_req_builder =
            local_var_req_builder.query(&[("show_profile_comments", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supporter_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("supporter_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supporter_since {
        local_var_req_builder =
            local_var_req_builder.query(&[("supporter_since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tachyons {
        local_var_req_builder =
            local_var_req_builder.query(&[("tachyons", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = url {
        local_var_req_builder = local_var_req_builder.query(&[("url", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username {
        local_var_req_builder =
            local_var_req_builder.query(&[("username", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username_change_cost {
        local_var_req_builder =
            local_var_req_builder.query(&[("username_change_cost", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&patched_user_profile);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserProfilesPartialUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_profiles_retrieve(
    configuration: &configuration::Configuration,
    params: UserProfilesRetrieveParams,
) -> Result<crate::models::UserProfile, Error<UserProfilesRetrieveError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/user-profiles/{id}/",
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
        let local_var_entity: Option<UserProfilesRetrieveError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn user_profiles_update(
    configuration: &configuration::Configuration,
    params: UserProfilesUpdateParams,
) -> Result<crate::models::UserProfile, Error<UserProfilesUpdateError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let first_name = params.first_name;
    let id = params.id;
    let last_name = params.last_name;
    let ask_when_reaffirm_question_modal = params.ask_when_reaffirm_question_modal;
    let date_joined = params.date_joined;
    let default_community_visibility = params.default_community_visibility;
    let default_mp_visibility = params.default_mp_visibility;
    let email = params.email;
    let formerly_known_as = params.formerly_known_as;
    let is_staff = params.is_staff;
    let is_superuser = params.is_superuser;
    let last_visited = params.last_visited;
    let level = params.level;
    let level_title = params.level_title;
    let permissions = params.permissions;
    let powers = params.powers;
    let purchasable_track_record = params.purchasable_track_record;
    let score = params.score;
    let show_profile_comments = params.show_profile_comments;
    let supporter_level = params.supporter_level;
    let supporter_since = params.supporter_since;
    let tachyons = params.tachyons;
    let url = params.url;
    let username = params.username;
    let username_change_cost = params.username_change_cost;
    let user_profile = params.user_profile;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api2/user-profiles/{id}/",
        local_var_configuration.base_path,
        id = id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ask_when_reaffirm_question_modal {
        local_var_req_builder = local_var_req_builder.query(&[(
            "ask_when_reaffirm_question_modal",
            &local_var_str.to_string(),
        )]);
    }
    if let Some(ref local_var_str) = date_joined {
        local_var_req_builder =
            local_var_req_builder.query(&[("date_joined", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = default_community_visibility {
        local_var_req_builder = local_var_req_builder
            .query(&[("default_community_visibility", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = default_mp_visibility {
        local_var_req_builder =
            local_var_req_builder.query(&[("default_mp_visibility", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = email {
        local_var_req_builder =
            local_var_req_builder.query(&[("email", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("first_name", &first_name.to_string())]);
    if let Some(ref local_var_str) = formerly_known_as {
        local_var_req_builder =
            local_var_req_builder.query(&[("formerly_known_as", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_staff {
        local_var_req_builder =
            local_var_req_builder.query(&[("is_staff", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_superuser {
        local_var_req_builder =
            local_var_req_builder.query(&[("is_superuser", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("last_name", &last_name.to_string())]);
    if let Some(ref local_var_str) = last_visited {
        local_var_req_builder =
            local_var_req_builder.query(&[("last_visited", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = level {
        local_var_req_builder =
            local_var_req_builder.query(&[("level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = level_title {
        local_var_req_builder =
            local_var_req_builder.query(&[("levelTitle", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = permissions {
        local_var_req_builder =
            local_var_req_builder.query(&[("permissions", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = powers {
        local_var_req_builder =
            local_var_req_builder.query(&[("powers", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = purchasable_track_record {
        local_var_req_builder = local_var_req_builder
            .query(&[("purchasable_track_record", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = score {
        local_var_req_builder =
            local_var_req_builder.query(&[("score", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_profile_comments {
        local_var_req_builder =
            local_var_req_builder.query(&[("show_profile_comments", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supporter_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("supporter_level", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = supporter_since {
        local_var_req_builder =
            local_var_req_builder.query(&[("supporter_since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tachyons {
        local_var_req_builder =
            local_var_req_builder.query(&[("tachyons", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = url {
        local_var_req_builder = local_var_req_builder.query(&[("url", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username {
        local_var_req_builder =
            local_var_req_builder.query(&[("username", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = username_change_cost {
        local_var_req_builder =
            local_var_req_builder.query(&[("username_change_cost", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&user_profile);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UserProfilesUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
