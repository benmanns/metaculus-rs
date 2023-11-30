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
pub struct CommentChildren {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<i32>,
    #[serde(rename = "author_name", deserialize_with = "Option::deserialize")]
    pub author_name: Option<String>,
    #[serde(rename = "author_lvl", deserialize_with = "Option::deserialize")]
    pub author_lvl: Option<serde_json::Value>,
    #[serde(rename = "author_supporter_lvl")]
    pub author_supporter_lvl: i32,
    #[serde(rename = "is_deactivated")]
    pub is_deactivated: bool,
    #[serde(rename = "is_moderator")]
    pub is_moderator: bool,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    #[serde(rename = "question")]
    pub question: String,
    #[serde(rename = "comment_html")]
    pub comment_html: String,
    #[serde(rename = "comment_text")]
    pub comment_text: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "prediction_value", deserialize_with = "Option::deserialize")]
    pub prediction_value: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "submit_type", skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<crate::models::SubmitTypeEnum>,
    #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(
        rename = "parent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Option<i32>>,
    #[serde(rename = "num_children")]
    pub num_children: i32,
    #[serde(rename = "num_likes", deserialize_with = "Option::deserialize")]
    pub num_likes: Option<i32>,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "parent_author", deserialize_with = "Option::deserialize")]
    pub parent_author: Option<String>,
    #[serde(rename = "user_like")]
    pub user_like: i32,
    #[serde(rename = "user_like_at")]
    pub user_like_at: String,
    #[serde(
        rename = "latest_prediction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_prediction: Option<Option<::std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(
        rename = "include_latest_prediction",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_latest_prediction: Option<bool>,
    #[serde(rename = "edits")]
    pub edits: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "children")]
    pub children: Box<crate::models::Comment>,
}

impl CommentChildren {
    #[must_use]
    pub fn new(
        url: String,
        id: i32,
        author: Option<i32>,
        author_name: Option<String>,
        author_lvl: Option<serde_json::Value>,
        author_supporter_lvl: i32,
        is_deactivated: bool,
        is_moderator: bool,
        is_admin: bool,
        question: String,
        comment_html: String,
        comment_text: String,
        created_time: String,
        prediction_value: Option<::std::collections::HashMap<String, serde_json::Value>>,
        num_children: i32,
        num_likes: Option<i32>,
        deleted: bool,
        parent_author: Option<String>,
        user_like: i32,
        user_like_at: String,
        edits: ::std::collections::HashMap<String, serde_json::Value>,
        children: crate::models::Comment,
    ) -> CommentChildren {
        CommentChildren {
            url,
            id,
            author,
            author_name,
            author_lvl,
            author_supporter_lvl,
            is_deactivated,
            is_moderator,
            is_admin,
            question,
            comment_html,
            comment_text,
            created_time,
            prediction_value,
            submit_type: None,
            preview: None,
            parent: None,
            num_children,
            num_likes,
            deleted,
            parent_author,
            user_like,
            user_like_at,
            latest_prediction: None,
            include_latest_prediction: None,
            edits,
            children: Box::new(children),
        }
    }
}
