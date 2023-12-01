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
pub struct ProjectUserStats {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "project")]
    pub project: i32,
    #[serde(rename = "points")]
    pub points: i32,
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "log_score")]
    pub log_score: f64,
    #[serde(rename = "num_questions")]
    pub num_questions: i32,
    #[serde(rename = "num_questions_predicted")]
    pub num_questions_predicted: i32,
    #[serde(rename = "prize")]
    pub prize: f64,
    #[serde(rename = "coverage_avg")]
    pub coverage_avg: f64,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "level_title")]
    pub level_title: String,
}

impl ProjectUserStats {
    #[must_use]
    pub fn new(
        id: i32,
        username: String,
        project: i32,
        points: i32,
        rank: i32,
        log_score: f64,
        num_questions: i32,
        num_questions_predicted: i32,
        prize: f64,
        coverage_avg: f64,
        level: i32,
        level_title: String,
    ) -> ProjectUserStats {
        ProjectUserStats {
            id,
            username,
            project,
            points,
            rank,
            log_score,
            num_questions,
            num_questions_predicted,
            prize,
            coverage_avg,
            level,
            level_title,
        }
    }
}
