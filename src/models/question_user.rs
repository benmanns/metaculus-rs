/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

/// `QuestionUser` : Just contains basic data used by all other serializers. Does not include all of the derived data.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionUser {
    #[serde(rename = "active_state", deserialize_with = "Option::deserialize")]
    pub active_state: Option<serde_json::Value>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "page_url")]
    pub page_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "author")]
    pub author: i32,
    #[serde(rename = "author_name")]
    pub author_name: String,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "title_short")]
    pub title_short: String,
    #[serde(rename = "group_label", skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::Status3baEnum>,
    #[serde(rename = "resolution", deserialize_with = "Option::deserialize")]
    pub resolution: Option<f64>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "publish_time", skip_serializing_if = "Option::is_none")]
    pub publish_time: Option<String>,
    #[serde(rename = "close_time", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(
        rename = "effected_close_time",
        deserialize_with = "Option::deserialize"
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
    #[serde(rename = "user_perms", deserialize_with = "Option::deserialize")]
    pub user_perms: Option<serde_json::Value>,
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
    #[serde(rename = "edited_time")]
    pub edited_time: String,
    #[serde(rename = "last_activity_time")]
    pub last_activity_time: String,
    #[serde(rename = "activity")]
    pub activity: f64,
    #[serde(rename = "comment_count")]
    pub comment_count: i32,
    #[serde(rename = "votes")]
    pub votes: i32,
    #[serde(
        rename = "metaculus_prediction",
        deserialize_with = "Option::deserialize"
    )]
    pub metaculus_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "community_prediction",
        deserialize_with = "Option::deserialize"
    )]
    pub community_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "number_of_forecasters",
        deserialize_with = "Option::deserialize"
    )]
    pub number_of_forecasters: Option<i32>,
    #[serde(rename = "prediction_count")]
    pub prediction_count: i32,
    #[serde(rename = "related_questions", deserialize_with = "Option::deserialize")]
    pub related_questions: Option<Vec<crate::models::QuestionRelated>>,
    #[serde(rename = "group", deserialize_with = "Option::deserialize")]
    pub group: Option<i32>,
    #[serde(rename = "condition", deserialize_with = "Option::deserialize")]
    pub condition: Option<serde_json::Value>,
    #[serde(rename = "sub_questions")]
    pub sub_questions: Vec<crate::models::SubQuestionUserList>,
    #[serde(rename = "options")]
    pub options: Vec<crate::models::Option>,
    #[serde(rename = "has_fan_graph")]
    pub has_fan_graph: bool,
    #[serde(rename = "projects", deserialize_with = "Option::deserialize")]
    pub projects: Option<serde_json::Value>,
    #[serde(
        rename = "community_absolute_log_score",
        deserialize_with = "Option::deserialize"
    )]
    pub community_absolute_log_score: Option<f64>,
    #[serde(
        rename = "metaculus_absolute_log_score",
        deserialize_with = "Option::deserialize"
    )]
    pub metaculus_absolute_log_score: Option<f64>,
    #[serde(
        rename = "metaculus_relative_log_score",
        deserialize_with = "Option::deserialize"
    )]
    pub metaculus_relative_log_score: Option<f64>,
    #[serde(
        rename = "unweighted_community_prediction_v2",
        deserialize_with = "Option::deserialize"
    )]
    pub unweighted_community_prediction_v2: Option<String>,
    #[serde(
        rename = "recency_weighted_community_prediction_v2",
        deserialize_with = "Option::deserialize"
    )]
    pub recency_weighted_community_prediction_v2: Option<String>,
    #[serde(rename = "comment_count_snapshot")]
    pub comment_count_snapshot: i32,
    #[serde(rename = "user_vote")]
    pub user_vote: i32,
    #[serde(rename = "user_community_vis")]
    pub user_community_vis: crate::models::UserCommunityVisEnum,
    #[serde(
        rename = "my_predictions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub my_predictions: Option<Option<Box<crate::models::ExtendedPredictionUsername>>>,
    #[serde(rename = "divergence", deserialize_with = "Option::deserialize")]
    pub divergence: Option<f64>,
    #[serde(rename = "peer_score", deserialize_with = "Option::deserialize")]
    pub peer_score: Option<f64>,
    #[serde(rename = "baseline_score", deserialize_with = "Option::deserialize")]
    pub baseline_score: Option<f64>,
    #[serde(
        rename = "user_prediction_sequence",
        deserialize_with = "Option::deserialize"
    )]
    pub user_prediction_sequence: Option<String>,
}

impl QuestionUser {
    /// Just contains basic data used by all other serializers. Does not include all of the derived data.
    #[must_use]
    pub fn new(
        active_state: Option<serde_json::Value>,
        url: String,
        page_url: String,
        id: i32,
        author: i32,
        author_name: String,
        title_short: String,
        resolution: Option<f64>,
        created_time: String,
        effected_close_time: Option<String>,
        user_perms: Option<serde_json::Value>,
        edited_time: String,
        last_activity_time: String,
        activity: f64,
        comment_count: i32,
        votes: i32,
        metaculus_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
        community_prediction: Option<::std::collections::HashMap<String, serde_json::Value>>,
        number_of_forecasters: Option<i32>,
        prediction_count: i32,
        related_questions: Option<Vec<crate::models::QuestionRelated>>,
        group: Option<i32>,
        condition: Option<serde_json::Value>,
        sub_questions: Vec<crate::models::SubQuestionUserList>,
        options: Vec<crate::models::Option>,
        has_fan_graph: bool,
        projects: Option<serde_json::Value>,
        community_absolute_log_score: Option<f64>,
        metaculus_absolute_log_score: Option<f64>,
        metaculus_relative_log_score: Option<f64>,
        unweighted_community_prediction_v2: Option<String>,
        recency_weighted_community_prediction_v2: Option<String>,
        comment_count_snapshot: i32,
        user_vote: i32,
        user_community_vis: crate::models::UserCommunityVisEnum,
        divergence: Option<f64>,
        peer_score: Option<f64>,
        baseline_score: Option<f64>,
        user_prediction_sequence: Option<String>,
    ) -> QuestionUser {
        QuestionUser {
            active_state,
            url,
            page_url,
            id,
            author,
            author_name,
            title: None,
            title_short,
            group_label: None,
            status: None,
            resolution,
            created_time,
            publish_time: None,
            close_time: None,
            effected_close_time,
            resolve_time: None,
            possibilities: None,
            scoring: None,
            r#type: None,
            user_perms,
            weekly_movement: None,
            weekly_movement_direction: None,
            cp_reveal_time: None,
            edited_time,
            last_activity_time,
            activity,
            comment_count,
            votes,
            metaculus_prediction,
            community_prediction,
            number_of_forecasters,
            prediction_count,
            related_questions,
            group,
            condition,
            sub_questions,
            options,
            has_fan_graph,
            projects,
            community_absolute_log_score,
            metaculus_absolute_log_score,
            metaculus_relative_log_score,
            unweighted_community_prediction_v2,
            recency_weighted_community_prediction_v2,
            comment_count_snapshot,
            user_vote,
            user_community_vis,
            my_predictions: None,
            divergence,
            peer_score,
            baseline_score,
            user_prediction_sequence,
        }
    }
}
