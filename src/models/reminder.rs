/*
 * Metaculus API
 *
 * Welcome to the unofficial Rust client for the Metaculus API
 *
 * The version of the OpenAPI document: 1.0
 * Contact: Benjamin Manns <opensource@benmanns.com>
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Reminder {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "reminder", skip_serializing_if = "Option::is_none")]
    pub reminder: Option<crate::models::ReminderEnum>,
    #[serde(rename = "question")]
    pub question: i32,
    #[serde(rename = "user")]
    pub user: i32,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ReminderStatusEnum>,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "question_title", deserialize_with = "Option::deserialize")]
    pub question_title: Option<String>,
    #[serde(rename = "status_label")]
    pub status_label: String,
    #[serde(rename = "reminder_label")]
    pub reminder_label: String,
    #[serde(
        rename = "remind_on_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remind_on_date: Option<Option<String>>,
    #[serde(
        rename = "remind_on_new_comments",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remind_on_new_comments: Option<Option<i32>>,
    #[serde(
        rename = "remind_base_prediction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remind_base_prediction:
        Option<Option<::std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(
        rename = "remind_on_percent_lifetime",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remind_on_percent_lifetime: Option<Option<i32>>,
    #[serde(
        rename = "next_trigger_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_trigger_date: Option<Option<String>>,
    #[serde(
        rename = "next_trigger_value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_trigger_value: Option<Option<i32>>,
    #[serde(rename = "repeat_pattern", skip_serializing_if = "Option::is_none")]
    pub repeat_pattern: Option<crate::models::RepeatPatternEnum>,
}

impl Reminder {
    #[must_use]
    pub fn new(
        id: i32,
        question: i32,
        user: i32,
        created_time: String,
        question_title: Option<String>,
        status_label: String,
        reminder_label: String,
    ) -> Reminder {
        Reminder {
            id,
            reminder: None,
            question,
            user,
            status: None,
            created_time,
            note: None,
            question_title,
            status_label,
            reminder_label,
            remind_on_date: None,
            remind_on_new_comments: None,
            remind_base_prediction: None,
            remind_on_percent_lifetime: None,
            next_trigger_date: None,
            next_trigger_value: None,
            repeat_pattern: None,
        }
    }
}
