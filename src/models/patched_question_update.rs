/*
 * Metaculus API
 *
 *  <b>Welcome to the official Metaculus API!</b>  If you have questions, ideas, or feedback, please contact our team at api-requests@metaculus.com. We are excited to keep building upon this initial version of the API, and we’d like to keep making it more useful to you. Our aim is to support the forecasting community – we’re listening!  <b>Get Started in 15 Seconds</b>  <ol> <li>Most of the API is (hopefully) self-explanatory. You’ll find all the documentation below. <li>If you’re testing the waters or doing a one-off analysis, you can dive right in! <li>If you’re building an application that connects to the Metaculus API, you’ll need to authenticate. </ol>  <b>How to Authenticate</b>  To request an auth token, email api-requests@metaculus.com and let Dan, Jon, and Martin know what you’d like to build. Please tell us a bit about yourself or your organization and how you intend to use the API.  You can then add the token we generate for you to your requests using the <i>Authorization</i> HTTP header. The token should be prefixed by the string  literal \"Token\", with whitespace separating the two strings.  Example:  <blockquote> Authorization: Token 9944b09199c62bcf9418ad846dd0e4bbdfc6ee4b </blockquote>  <b>A Note for Early Adopters of the API from 2020-2023</b>  Please be aware that we‘re rapidly improving our forecasting tools and so cannot guarantee continued backward compatibility for all features. If you previously authenticated via cookies, we will continue to support this functionality for now, but we ask that you please switch to using auth tokens as described above by June 1, 2023.  We're eager to support wider usage of the API, and the more feedback we get from the community, the more helpful we can be. (For certain endpoints, access levels vary depending on user permissions.)  Please send us an email at api-requests@metaculus.com if you’d like to discuss your specific application.  <i>Updated as of: May 1, 2023</i>  <hr/> <h3 id=\"tutorial\"> A Very Quick Tutorial</h3>  Let's walk through the process of fetching a list of questions, getting the details of an individual question, and making a prediction.  <b>Fetching a Question List and Question Details</b>  This is straightforward: we set the <i>Authorization</i> header as described above, and fetch the URL  <i>https://www.metaculus.com/api2/questions/</i>  Let's make it a little more interesting by only fetching questions which were asked during  and resolved before the end of, 2022. We can add the query parameters  <i>publish_time__gt</i> and <i>resolve_time__lt</i> accordingly  (note double underscores before <i>gt</i> and </i>let</i>):  <i>https://www.metaculus.com/api2/questions/?publish_time__gt=2021-12-31&resolve_time__lt=2023-01-01</i>  (We could make it more precise by saying e.g. <i>public_time__t=2021-12-31T23:59:59Z</i> but this is sufficient for our purposes.) This gets us the question with ID 7871, <i>\"Will there be a female Fields Medalist in 2022?\"</i>  In practice, the data from <i>/questions</i> includes almost all the data about a question -- and even more, such as the description and its categories, if you also add the query parameter <i>include_description=true</i>. But you could easily get the few additional details from  <i>https://www.metaculus.com/api2/questions/7871</i>  <b>Making a Prediction</b>  We can't <i>actually</i> post a new prediction for that question, since it has resolved. But if we were to time-travel back to before its resolution, we would do so by simply HTTP POSTing to the URL  <i>https://www.metaculus.com/api2/questions/7871/predict</i>  the payload  {     \"prediction\": 0.9, }  to predict a 90% chance of this happening. If we <i>did</i> have the permissions required to make that prediction, we would get back a response with the prediction we just made. Note that this is a binary (i.e. yes/no) question; for a <i>continuous</i> question, the payload gets a little more complicated.  <b>Making A Continuous Prediction</b>  ...OK: a <i>lot</i> more complicated. For now, we'll just document the JSON schema that such predictions require below, and will document its usage more thoroughly it in a separate subsequent tutorial. In the interim, please contact us at <a href=\"mailto:api-requests@metaculus.com\">api-requests@metaculus.com</a> if you wish to use the API to make continuous predictions.  <b>JSON Schema for Continuous Predictions</b>  <blockquote> schema = {     \"type\": \"object\",     \"properties\": {         \"kind\": {             \"enum\": ([\"logistic\", \"gaussian\"])         },         \"avg\": {             \"type\": \"number\",             \"minimum\": -2,             \"maximum\": 3,         },         \"stdev\": {             \"type\": \"number\",             \"minimum\": 0.005,             \"maximum\": 10,         },         \"a\": {             \"type\": \"number\",             \"minimum\": -0.96,             \"maximum\": +0.96,         },         \"low\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },         \"high\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },     },     \"additionalProperties\": boolean,     \"required\": [\"avg\", \"stdev\"] } </blockquote> For questions with lower/upper bounds, \"low\"/\"high\" are required. For logistic distributions, the schema requires \"x0\" and \"s\" rather than \"avg\" and \"stdev.\" Metaculus supports multiple superimposed distributions, in which case the above schema will be nested in a \"super-schema\" containing each of those; contact us for details. <hr/>
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

/// PatchedQuestionUpdate : Just contains basic data used by all other serializers. Does not include all of the derived data.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedQuestionUpdate {
    #[serde(
        rename = "active_state",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_state: Option<Option<serde_json::Value>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "page_url", skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<i32>,
    #[serde(rename = "author_name", skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_short", skip_serializing_if = "Option::is_none")]
    pub title_short: Option<String>,
    #[serde(rename = "group_label", skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::QuestionUpdateStatusEnum>,
    #[serde(
        rename = "resolution",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution: Option<Option<f64>>,
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "publish_time", skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(rename = "close_time", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(
        rename = "effected_close_time",
        skip_serializing_if = "Option::is_none"
    )]
    pub effected_close_time: Option<String>,
    #[serde(rename = "resolve_time", skip_serializing_if = "Option::is_none")]
    pub resolve_time: Option<String>,
    #[serde(rename = "possibilities", skip_serializing_if = "Option::is_none")]
    pub possibilities: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "scoring", skip_serializing_if = "Option::is_none")]
    pub scoring: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::QuestionTypes>,
    #[serde(
        rename = "user_perms",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_perms: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "weekly_movement",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_movement: Option<Option<f64>>,
    #[serde(
        rename = "weekly_movement_direction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_movement_direction: Option<Option<i32>>,
    #[serde(rename = "cp_reveal_time", skip_serializing_if = "Option::is_none")]
    pub cp_reveal_time: Option<String>,
    #[serde(rename = "edited_time", skip_serializing_if = "Option::is_none")]
    pub edited_time: Option<String>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "description_html", skip_serializing_if = "Option::is_none")]
    pub description_html: Option<String>,
    #[serde(
        rename = "resolution_criteria",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_criteria: Option<String>,
    #[serde(
        rename = "resolution_criteria_html",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_criteria_html: Option<String>,
    #[serde(rename = "fine_print", skip_serializing_if = "Option::is_none")]
    pub fine_print: Option<String>,
    #[serde(rename = "fine_print_html", skip_serializing_if = "Option::is_none")]
    pub fine_print_html: Option<String>,
    #[serde(rename = "prediction_count", skip_serializing_if = "Option::is_none")]
    pub prediction_count: Option<i32>,
    #[serde(rename = "parent_question_id", skip_serializing_if = "Option::is_none")]
    pub parent_question_id: Option<i32>,
    #[serde(rename = "related_questions", skip_serializing_if = "Option::is_none")]
    pub related_questions: Option<Vec<serde_json::Value>>,
    #[serde(rename = "sub_questions", skip_serializing_if = "Option::is_none")]
    pub sub_questions: Option<Vec<crate::models::SubQuestionUpdate>>,
    #[serde(
        rename = "projects",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub projects: Option<Option<serde_json::Value>>,
    #[serde(rename = "edit_form_data", skip_serializing_if = "Option::is_none")]
    pub edit_form_data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "projects_input", skip_serializing_if = "Option::is_none")]
    pub projects_input: Option<Vec<crate::models::QuestionProject>>,
    #[serde(rename = "conditioned_on", skip_serializing_if = "Option::is_none")]
    pub conditioned_on: Option<i32>,
    #[serde(
        rename = "unconditional_question",
        skip_serializing_if = "Option::is_none"
    )]
    pub unconditional_question: Option<i32>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::Option>>,
}

impl PatchedQuestionUpdate {
    /// Just contains basic data used by all other serializers. Does not include all of the derived data.
    pub fn new() -> PatchedQuestionUpdate {
        PatchedQuestionUpdate {
            active_state: None,
            url: None,
            page_url: None,
            id: None,
            author: None,
            author_name: None,
            title: None,
            title_short: None,
            group_label: None,
            status: None,
            resolution: None,
            created_time: None,
            publish_time: None,
            close_time: None,
            effected_close_time: None,
            resolve_time: None,
            possibilities: None,
            scoring: None,
            r#type: None,
            user_perms: None,
            weekly_movement: None,
            weekly_movement_direction: None,
            cp_reveal_time: None,
            edited_time: None,
            categories: None,
            description: None,
            description_html: None,
            resolution_criteria: None,
            resolution_criteria_html: None,
            fine_print: None,
            fine_print_html: None,
            prediction_count: None,
            parent_question_id: None,
            related_questions: None,
            sub_questions: None,
            projects: None,
            edit_form_data: None,
            projects_input: None,
            conditioned_on: None,
            unconditional_question: None,
            options: None,
        }
    }
}