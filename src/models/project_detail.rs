/*
 * Metaculus API
 *
 *  <b>Welcome to the official Metaculus API!</b>  If you have questions, ideas, or feedback, please contact our team at api-requests@metaculus.com. We are excited to keep building upon this initial version of the API, and we’d like to keep making it more useful to you. Our aim is to support the forecasting community – we’re listening!  <b>Get Started in 15 Seconds</b>  <ol> <li>Most of the API is (hopefully) self-explanatory. You’ll find all the documentation below. <li>If you’re testing the waters or doing a one-off analysis, you can dive right in! <li>If you’re building an application that connects to the Metaculus API, you’ll need to authenticate. </ol>  <b>How to Authenticate</b>  To request an auth token, email api-requests@metaculus.com and let Dan, Jon, and Martin know what you’d like to build. Please tell us a bit about yourself or your organization and how you intend to use the API.  You can then add the token we generate for you to your requests using the <i>Authorization</i> HTTP header. The token should be prefixed by the string  literal \"Token\", with whitespace separating the two strings.  Example:  <blockquote> Authorization: Token 9944b09199c62bcf9418ad846dd0e4bbdfc6ee4b </blockquote>  <b>A Note for Early Adopters of the API from 2020-2023</b>  Please be aware that we‘re rapidly improving our forecasting tools and so cannot guarantee continued backward compatibility for all features. If you previously authenticated via cookies, we will continue to support this functionality for now, but we ask that you please switch to using auth tokens as described above by June 1, 2023.  We're eager to support wider usage of the API, and the more feedback we get from the community, the more helpful we can be. (For certain endpoints, access levels vary depending on user permissions.)  Please send us an email at api-requests@metaculus.com if you’d like to discuss your specific application.  <i>Updated as of: May 1, 2023</i>  <hr/> <h3 id=\"tutorial\"> A Very Quick Tutorial</h3>  Let's walk through the process of fetching a list of questions, getting the details of an individual question, and making a prediction.  <b>Fetching a Question List and Question Details</b>  This is straightforward: we set the <i>Authorization</i> header as described above, and fetch the URL  <i>https://www.metaculus.com/api2/questions/</i>  Let's make it a little more interesting by only fetching questions which were asked during  and resolved before the end of, 2022. We can add the query parameters  <i>publish_time__gt</i> and <i>resolve_time__lt</i> accordingly  (note double underscores before <i>gt</i> and </i>let</i>):  <i>https://www.metaculus.com/api2/questions/?publish_time__gt=2021-12-31&resolve_time__lt=2023-01-01</i>  (We could make it more precise by saying e.g. <i>public_time__t=2021-12-31T23:59:59Z</i> but this is sufficient for our purposes.) This gets us the question with ID 7871, <i>\"Will there be a female Fields Medalist in 2022?\"</i>  In practice, the data from <i>/questions</i> includes almost all the data about a question -- and even more, such as the description and its categories, if you also add the query parameter <i>include_description=true</i>. But you could easily get the few additional details from  <i>https://www.metaculus.com/api2/questions/7871</i>  <b>Making a Prediction</b>  We can't <i>actually</i> post a new prediction for that question, since it has resolved. But if we were to time-travel back to before its resolution, we would do so by simply HTTP POSTing to the URL  <i>https://www.metaculus.com/api2/questions/7871/predict</i>  the payload  {     \"prediction\": 0.9, }  to predict a 90% chance of this happening. If we <i>did</i> have the permissions required to make that prediction, we would get back a response with the prediction we just made. Note that this is a binary (i.e. yes/no) question; for a <i>continuous</i> question, the payload gets a little more complicated.  <b>Making A Continuous Prediction</b>  ...OK: a <i>lot</i> more complicated. For now, we'll just document the JSON schema that such predictions require below, and will document its usage more thoroughly it in a separate subsequent tutorial. In the interim, please contact us at <a href=\"mailto:api-requests@metaculus.com\">api-requests@metaculus.com</a> if you wish to use the API to make continuous predictions.  <b>JSON Schema for Continuous Predictions</b>  <blockquote> schema = {     \"type\": \"object\",     \"properties\": {         \"kind\": {             \"enum\": ([\"logistic\", \"gaussian\"])         },         \"avg\": {             \"type\": \"number\",             \"minimum\": -2,             \"maximum\": 3,         },         \"stdev\": {             \"type\": \"number\",             \"minimum\": 0.005,             \"maximum\": 10,         },         \"a\": {             \"type\": \"number\",             \"minimum\": -0.96,             \"maximum\": +0.96,         },         \"low\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },         \"high\": {             \"type\": \"number\",             \"minimum\": 0.0099,             \"maximum\": 1 - 0.0099,         },     },     \"additionalProperties\": boolean,     \"required\": [\"avg\", \"stdev\"] } </blockquote> For questions with lower/upper bounds, \"low\"/\"high\" are required. For logistic distributions, the schema requires \"x0\" and \"s\" rather than \"avg\" and \"stdev.\" Metaculus supports multiple superimposed distributions, in which case the above schema will be nested in a \"super-schema\" containing each of those; contact us for details. <hr/>
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDetail {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "type_str")]
    pub type_str: String,
    #[serde(rename = "score_type")]
    pub score_type: crate::models::ScoreTypeEnum,
    #[serde(rename = "subtitle")]
    pub subtitle: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "site_id")]
    pub site_id: i32,
    #[serde(rename = "user_perms", deserialize_with = "Option::deserialize")]
    pub user_perms: Option<serde_json::Value>,
    #[serde(
        rename = "question_user_perms",
        deserialize_with = "Option::deserialize"
    )]
    pub question_user_perms: Option<serde_json::Value>,
    #[serde(rename = "sign_up_fields")]
    pub sign_up_fields: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "tournament_close_date")]
    pub tournament_close_date: String,
    #[serde(rename = "tournament_start_date")]
    pub tournament_start_date: String,
    #[serde(rename = "prize_pool")]
    pub prize_pool: f64,
    #[serde(rename = "config")]
    pub config: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "in_main_feed_by_default")]
    pub in_main_feed_by_default: bool,
    #[serde(rename = "public")]
    pub public: bool,
    #[serde(rename = "is_opt_in")]
    pub is_opt_in: bool,
    #[serde(rename = "user_permissions", deserialize_with = "Option::deserialize")]
    pub user_permissions: Option<serde_json::Value>,
    #[serde(rename = "essay_count")]
    pub essay_count: i32,
    #[serde(rename = "completion_question_count")]
    pub completion_question_count: i32,
    #[serde(rename = "organizations")]
    pub organizations: Vec<crate::models::Organization>,
    #[serde(rename = "header_logo")]
    pub header_logo: String,
    #[serde(rename = "header_image")]
    pub header_image: String,
    #[serde(
        rename = "default_project_permissions",
        deserialize_with = "Option::deserialize"
    )]
    pub default_project_permissions: Option<serde_json::Value>,
    #[serde(
        rename = "default_question_permissions",
        deserialize_with = "Option::deserialize"
    )]
    pub default_question_permissions: Option<serde_json::Value>,
    #[serde(rename = "default_role", deserialize_with = "Option::deserialize")]
    pub default_role: Option<serde_json::Value>,
}

impl ProjectDetail {
    #[must_use]
    pub fn new(
        id: i32,
        name: String,
        url: String,
        type_str: String,
        score_type: crate::models::ScoreTypeEnum,
        subtitle: String,
        description: String,
        r#type: String,
        site_id: i32,
        user_perms: Option<serde_json::Value>,
        question_user_perms: Option<serde_json::Value>,
        sign_up_fields: ::std::collections::HashMap<String, serde_json::Value>,
        tournament_close_date: String,
        tournament_start_date: String,
        prize_pool: f64,
        config: ::std::collections::HashMap<String, serde_json::Value>,
        in_main_feed_by_default: bool,
        public: bool,
        is_opt_in: bool,
        user_permissions: Option<serde_json::Value>,
        essay_count: i32,
        completion_question_count: i32,
        organizations: Vec<crate::models::Organization>,
        header_logo: String,
        header_image: String,
        default_project_permissions: Option<serde_json::Value>,
        default_question_permissions: Option<serde_json::Value>,
        default_role: Option<serde_json::Value>,
    ) -> ProjectDetail {
        ProjectDetail {
            id,
            name,
            url,
            type_str,
            score_type,
            subtitle,
            description,
            r#type,
            site_id,
            user_perms,
            question_user_perms,
            sign_up_fields,
            tournament_close_date,
            tournament_start_date,
            prize_pool,
            config,
            in_main_feed_by_default,
            public,
            is_opt_in,
            user_permissions,
            essay_count,
            completion_question_count,
            organizations,
            header_logo,
            header_image,
            default_project_permissions,
            default_question_permissions,
            default_role,
        }
    }
}
