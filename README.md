# Rust API client for metaculus


<b>Welcome to the official Metaculus API!</b>

If you have questions, ideas, or feedback, please contact our team at
api-requests@metaculus.com. We are excited to keep building upon this
initial version of the API, and we’d like to keep making it more useful
to you. Our aim is to support the forecasting community – we’re listening!

<b>Get Started in 15 Seconds</b>

<ol>
<li>Most of the API is (hopefully) self-explanatory. You’ll find all the documentation below.
<li>If you’re testing the waters or doing a one-off analysis, you can dive right in!
<li>If you’re building an application that connects to the Metaculus API, you’ll need to authenticate.
</ol>

<b>How to Authenticate</b>

To request an auth token, email api-requests@metaculus.com and let Dan, Jon,
and Martin know what you’d like to build. Please tell us a bit about yourself
or your organization and how you intend to use the API.

You can then add the token we generate for you to your requests using the
<i>Authorization</i> HTTP header. The token should be prefixed by the string
 literal \"Token\", with whitespace separating the two strings.

Example:

<blockquote>
Authorization: Token 9944b09199c62bcf9418ad846dd0e4bbdfc6ee4b
</blockquote>

<b>A Note for Early Adopters of the API from 2020-2023</b>

Please be aware that we‘re rapidly improving our forecasting tools and so cannot
guarantee continued backward compatibility for all features. If you previously
authenticated via cookies, we will continue to support this functionality for
now, but we ask that you please switch to using auth tokens as described above
by June 1, 2023.

We're eager to support wider usage of the API, and the more feedback we get from
the community, the more helpful we can be. (For certain endpoints, access levels
vary depending on user permissions.)

Please send us an email at api-requests@metaculus.com if you’d like to discuss your specific application.

<i>Updated as of: May 1, 2023</i>

<hr/>
<h3 id=\"tutorial\"> A Very Quick Tutorial</h3>

Let's walk through the process of fetching a list of questions, getting the details of an
individual question, and making a prediction.

<b>Fetching a Question List and Question Details</b>

This is straightforward: we set the <i>Authorization</i> header as described above, and fetch the URL

<i>https://www.metaculus.com/api2/questions/</i>

Let's make it a little more interesting by only fetching questions which were asked during
 and resolved before the end of, 2022. We can add the query parameters
 <i>publish_time__gt</i> and <i>resolve_time__lt</i> accordingly
 (note double underscores before <i>gt</i> and </i>let</i>):

<i>https://www.metaculus.com/api2/questions/?publish_time__gt=2021-12-31&resolve_time__lt=2023-01-01</i>

(We could make it more precise by saying e.g. <i>public_time__t=2021-12-31T23:59:59Z</i> but this is
sufficient for our purposes.) This gets us the question with ID 7871, <i>\"Will there be a female Fields
Medalist in 2022?\"</i>

In practice, the data from <i>/questions</i> includes almost all the data about
a question -- and even more, such as the description and its categories, if you also add the query
parameter <i>include_description=true</i>. But you could easily get the few additional details from

<i>https://www.metaculus.com/api2/questions/7871</i>

<b>Making a Prediction</b>

We can't <i>actually</i> post a new prediction for that question, since it has resolved. But if we
were to time-travel back to before its resolution, we would do so by simply HTTP POSTing to the URL

<i>https://www.metaculus.com/api2/questions/7871/predict</i>

the payload

{
    \"prediction\": 0.9,
}

to predict a 90% chance of this happening. If we <i>did</i> have the permissions required to make
that prediction, we would get back a response with the prediction we just made. Note that this is
a binary (i.e. yes/no) question; for a <i>continuous</i> question, the payload gets a little more
complicated.

<b>Making A Continuous Prediction</b>

...OK: a <i>lot</i> more complicated. For now, we'll just document the JSON schema that such
predictions require below, and will document its usage more thoroughly it in a separate subsequent
tutorial. In the interim, please contact us at
<a href=\"mailto:api-requests@metaculus.com\">api-requests@metaculus.com</a>
if you wish to use the API to make continuous predictions.

<b>JSON Schema for Continuous Predictions</b>

<blockquote>
schema = {
    \"type\": \"object\",
    \"properties\": {
        \"kind\": {
            \"enum\": ([\"logistic\", \"gaussian\"])
        },
        \"avg\": {
            \"type\": \"number\",
            \"minimum\": -2,
            \"maximum\": 3,
        },
        \"stdev\": {
            \"type\": \"number\",
            \"minimum\": 0.005,
            \"maximum\": 10,
        },
        \"a\": {
            \"type\": \"number\",
            \"minimum\": -0.96,
            \"maximum\": +0.96,
        },
        \"low\": {
            \"type\": \"number\",
            \"minimum\": 0.0099,
            \"maximum\": 1 - 0.0099,
        },
        \"high\": {
            \"type\": \"number\",
            \"minimum\": 0.0099,
            \"maximum\": 1 - 0.0099,
        },
    },
    \"additionalProperties\": boolean,
    \"required\": [\"avg\", \"stdev\"]
}
</blockquote>
For questions with lower/upper bounds, \"low\"/\"high\" are required. For logistic
distributions, the schema requires \"x0\" and \"s\" rather than \"avg\" and \"stdev.\"
Metaculus supports multiple superimposed distributions, in which case the above schema
will be nested in a \"super-schema\" containing each of those; contact us for details.
<hr/>



## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0
- Package version: 0.0.1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `metaculus` and add the following to `Cargo.toml` under `[dependencies]`:

```
metaculus = { path = "./metaculus" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AboutNumbersApi* | [**about_numbers_retrieve**](docs/AboutNumbersApi.md#about_numbers_retrieve) | **GET** /api2/about-numbers/ | 
*CategoriesApi* | [**categories_list**](docs/CategoriesApi.md#categories_list) | **GET** /api2/categories/ | 
*CategoriesApi* | [**categories_retrieve**](docs/CategoriesApi.md#categories_retrieve) | **GET** /api2/categories/{bare_id}/ | 
*CommentsApi* | [**comments_create**](docs/CommentsApi.md#comments_create) | **POST** /api2/comments/ | 
*CommentsApi* | [**comments_destroy**](docs/CommentsApi.md#comments_destroy) | **DELETE** /api2/comments/{id}/ | 
*CommentsApi* | [**comments_like_create**](docs/CommentsApi.md#comments_like_create) | **POST** /api2/comments/{id}/like/ | 
*CommentsApi* | [**comments_list**](docs/CommentsApi.md#comments_list) | **GET** /api2/comments/ | 
*CommentsApi* | [**comments_partial_update**](docs/CommentsApi.md#comments_partial_update) | **PATCH** /api2/comments/{id}/ | 
*CommentsApi* | [**comments_report_create**](docs/CommentsApi.md#comments_report_create) | **POST** /api2/comments/{id}/report/ | 
*CommentsApi* | [**comments_retrieve**](docs/CommentsApi.md#comments_retrieve) | **GET** /api2/comments/{id}/ | 
*CommentsApi* | [**comments_update**](docs/CommentsApi.md#comments_update) | **PUT** /api2/comments/{id}/ | 
*NotificationsApi* | [**notifications_list**](docs/NotificationsApi.md#notifications_list) | **GET** /api2/notifications/ | 
*NotificationsApi* | [**notifications_mark_read_create**](docs/NotificationsApi.md#notifications_mark_read_create) | **POST** /api2/notifications/mark_read/ | 
*OrganizationsApi* | [**organizations_list**](docs/OrganizationsApi.md#organizations_list) | **GET** /api2/organizations/ | 
*OrganizationsApi* | [**organizations_members_create**](docs/OrganizationsApi.md#organizations_members_create) | **POST** /api2/organizations/{id}/members/ | 
*OrganizationsApi* | [**organizations_members_create2**](docs/OrganizationsApi.md#organizations_members_create2) | **POST** /api2/organizations/{id}/members/{user_id}/ | 
*OrganizationsApi* | [**organizations_members_destroy**](docs/OrganizationsApi.md#organizations_members_destroy) | **DELETE** /api2/organizations/{id}/members/{user_id}/ | 
*OrganizationsApi* | [**organizations_members_partial_update**](docs/OrganizationsApi.md#organizations_members_partial_update) | **PATCH** /api2/organizations/{id}/members/{user_id}/ | 
*OrganizationsApi* | [**organizations_members_update**](docs/OrganizationsApi.md#organizations_members_update) | **PUT** /api2/organizations/{id}/members/{user_id}/ | 
*OrganizationsApi* | [**organizations_retrieve**](docs/OrganizationsApi.md#organizations_retrieve) | **GET** /api2/organizations/{id}/ | 
*PredictionsApi* | [**predictions_list**](docs/PredictionsApi.md#predictions_list) | **GET** /api2/predictions/ | 
*PredictionsApi* | [**predictions_retrieve**](docs/PredictionsApi.md#predictions_retrieve) | **GET** /api2/predictions/{id}/ | 
*ProjectsApi* | [**projects_create**](docs/ProjectsApi.md#projects_create) | **POST** /api2/projects/ | 
*ProjectsApi* | [**projects_follow_create**](docs/ProjectsApi.md#projects_follow_create) | **POST** /api2/projects/{id}/follow/ | 
*ProjectsApi* | [**projects_invite_members_create**](docs/ProjectsApi.md#projects_invite_members_create) | **POST** /api2/projects/{id}/invite-members/ | 
*ProjectsApi* | [**projects_is_following_retrieve**](docs/ProjectsApi.md#projects_is_following_retrieve) | **GET** /api2/projects/{id}/is-following/ | 
*ProjectsApi* | [**projects_join_create**](docs/ProjectsApi.md#projects_join_create) | **POST** /api2/projects/{id}/join/ | 
*ProjectsApi* | [**projects_leave_create**](docs/ProjectsApi.md#projects_leave_create) | **POST** /api2/projects/{id}/leave/ | 
*ProjectsApi* | [**projects_list**](docs/ProjectsApi.md#projects_list) | **GET** /api2/projects/ | 
*ProjectsApi* | [**projects_members_create**](docs/ProjectsApi.md#projects_members_create) | **POST** /api2/projects/{id}/members/{user_id}/ | 
*ProjectsApi* | [**projects_members_destroy**](docs/ProjectsApi.md#projects_members_destroy) | **DELETE** /api2/projects/{id}/members/{user_id}/ | 
*ProjectsApi* | [**projects_members_partial_update**](docs/ProjectsApi.md#projects_members_partial_update) | **PATCH** /api2/projects/{id}/members/{user_id}/ | 
*ProjectsApi* | [**projects_members_update**](docs/ProjectsApi.md#projects_members_update) | **PUT** /api2/projects/{id}/members/{user_id}/ | 
*ProjectsApi* | [**projects_partial_update**](docs/ProjectsApi.md#projects_partial_update) | **PATCH** /api2/projects/{id}/ | 
*ProjectsApi* | [**projects_personal_stats_retrieve**](docs/ProjectsApi.md#projects_personal_stats_retrieve) | **GET** /api2/projects/{id}/personal-stats/ | 
*ProjectsApi* | [**projects_register_create**](docs/ProjectsApi.md#projects_register_create) | **POST** /api2/projects/{id}/register/ | 
*ProjectsApi* | [**projects_registered_retrieve**](docs/ProjectsApi.md#projects_registered_retrieve) | **GET** /api2/projects/{id}/registered/ | 
*ProjectsApi* | [**projects_retrieve**](docs/ProjectsApi.md#projects_retrieve) | **GET** /api2/projects/{id}/ | 
*ProjectsApi* | [**projects_unfollow_create**](docs/ProjectsApi.md#projects_unfollow_create) | **POST** /api2/projects/{id}/unfollow/ | 
*ProjectsApi* | [**projects_update**](docs/ProjectsApi.md#projects_update) | **PUT** /api2/projects/{id}/ | 
*ProjectstatsApi* | [**projectstats_list**](docs/ProjectstatsApi.md#projectstats_list) | **GET** /api2/projectstats/ | 
*ProjectstatsApi* | [**projectstats_retrieve**](docs/ProjectstatsApi.md#projectstats_retrieve) | **GET** /api2/projectstats/{id}/ | 
*QuestionSummariesApi* | [**question_summaries_feedback_create**](docs/QuestionSummariesApi.md#question_summaries_feedback_create) | **POST** /api2/question-summaries/feedback/ | 
*QuestionSummariesApi* | [**question_summaries_retrieve**](docs/QuestionSummariesApi.md#question_summaries_retrieve) | **GET** /api2/question-summaries/{id}/ | 
*QuestionsApi* | [**questions_add_consideration_vote_create**](docs/QuestionsApi.md#questions_add_consideration_vote_create) | **POST** /api2/questions/{id}/add_consideration_vote/ | 
*QuestionsApi* | [**questions_boost_create**](docs/QuestionsApi.md#questions_boost_create) | **POST** /api2/questions/{id}/boost/ | 
*QuestionsApi* | [**questions_bulk_predict_create**](docs/QuestionsApi.md#questions_bulk_predict_create) | **POST** /api2/questions/bulk-predict/ | 
*QuestionsApi* | [**questions_create**](docs/QuestionsApi.md#questions_create) | **POST** /api2/questions/ | 
*QuestionsApi* | [**questions_destroy**](docs/QuestionsApi.md#questions_destroy) | **DELETE** /api2/questions/{id}/ | 
*QuestionsApi* | [**questions_list**](docs/QuestionsApi.md#questions_list) | **GET** /api2/questions/ | 
*QuestionsApi* | [**questions_partial_update**](docs/QuestionsApi.md#questions_partial_update) | **PATCH** /api2/questions/{id}/ | 
*QuestionsApi* | [**questions_predict_create**](docs/QuestionsApi.md#questions_predict_create) | **POST** /api2/questions/{id}/predict/ | 
*QuestionsApi* | [**questions_prediction_for_date_retrieve**](docs/QuestionsApi.md#questions_prediction_for_date_retrieve) | **GET** /api2/questions/{id}/prediction-for-date/ | 
*QuestionsApi* | [**questions_prediction_history_retrieve**](docs/QuestionsApi.md#questions_prediction_history_retrieve) | **GET** /api2/questions/{id}/prediction-history/ | 
*QuestionsApi* | [**questions_predictions_retrieve**](docs/QuestionsApi.md#questions_predictions_retrieve) | **GET** /api2/questions/{id}/predictions/ | 
*QuestionsApi* | [**questions_question_sharing_create**](docs/QuestionsApi.md#questions_question_sharing_create) | **POST** /api2/questions/{id}/question-sharing/{username}/ | 
*QuestionsApi* | [**questions_question_sharing_destroy**](docs/QuestionsApi.md#questions_question_sharing_destroy) | **DELETE** /api2/questions/{id}/question-sharing/{username}/ | 
*QuestionsApi* | [**questions_remove_consideration_vote_create**](docs/QuestionsApi.md#questions_remove_consideration_vote_create) | **POST** /api2/questions/{id}/remove_consideration_vote/ | 
*QuestionsApi* | [**questions_resolve_create**](docs/QuestionsApi.md#questions_resolve_create) | **POST** /api2/questions/{id}/resolve/ | 
*QuestionsApi* | [**questions_retrieve**](docs/QuestionsApi.md#questions_retrieve) | **GET** /api2/questions/{id}/ | 
*QuestionsApi* | [**questions_show_community_create**](docs/QuestionsApi.md#questions_show_community_create) | **POST** /api2/questions/{id}/show-community/ | 
*QuestionsApi* | [**questions_update**](docs/QuestionsApi.md#questions_update) | **PUT** /api2/questions/{id}/ | 
*QuestionsApi* | [**questions_view_metaculus_prediction_create**](docs/QuestionsApi.md#questions_view_metaculus_prediction_create) | **POST** /api2/questions/{id}/view-metaculus-prediction/ | 
*QuestionsApi* | [**questions_vote_create**](docs/QuestionsApi.md#questions_vote_create) | **POST** /api2/questions/{id}/vote/ | 
*RankingsApi* | [**rankings_list**](docs/RankingsApi.md#rankings_list) | **GET** /api2/rankings/ | 
*RankingsApi* | [**rankings_retrieve**](docs/RankingsApi.md#rankings_retrieve) | **GET** /api2/rankings/{id}/ | 
*RemindersApi* | [**reminders_create**](docs/RemindersApi.md#reminders_create) | **POST** /api2/reminders/ | 
*RemindersApi* | [**reminders_list**](docs/RemindersApi.md#reminders_list) | **GET** /api2/reminders/ | 
*RemindersApi* | [**reminders_partial_update**](docs/RemindersApi.md#reminders_partial_update) | **PATCH** /api2/reminders/{id}/ | 
*RemindersApi* | [**reminders_retrieve**](docs/RemindersApi.md#reminders_retrieve) | **GET** /api2/reminders/{id}/ | 
*RemindersApi* | [**reminders_update**](docs/RemindersApi.md#reminders_update) | **PUT** /api2/reminders/{id}/ | 
*StatsApi* | [**stats_user_feedback_create**](docs/StatsApi.md#stats_user_feedback_create) | **POST** /stats/user_feedback/ | 
*UserProfilesApi* | [**user_profiles_list**](docs/UserProfilesApi.md#user_profiles_list) | **GET** /api2/user-profiles/ | 
*UserProfilesApi* | [**user_profiles_partial_update**](docs/UserProfilesApi.md#user_profiles_partial_update) | **PATCH** /api2/user-profiles/{id}/ | 
*UserProfilesApi* | [**user_profiles_retrieve**](docs/UserProfilesApi.md#user_profiles_retrieve) | **GET** /api2/user-profiles/{id}/ | 
*UserProfilesApi* | [**user_profiles_update**](docs/UserProfilesApi.md#user_profiles_update) | **PUT** /api2/user-profiles/{id}/ | 
*UsersApi* | [**users_collect_tachyons_create**](docs/UsersApi.md#users_collect_tachyons_create) | **POST** /api2/users/{id}/collect-tachyons/ | 
*UsersApi* | [**users_global_cp_reminder_create**](docs/UsersApi.md#users_global_cp_reminder_create) | **POST** /api2/users/global-cp-reminder/ | 
*UsersApi* | [**users_global_cp_reminder_retrieve**](docs/UsersApi.md#users_global_cp_reminder_retrieve) | **GET** /api2/users/global-cp-reminder/ | 
*UsersApi* | [**users_list**](docs/UsersApi.md#users_list) | **GET** /api2/users/ | 
*UsersApi* | [**users_partial_update**](docs/UsersApi.md#users_partial_update) | **PATCH** /api2/users/{id}/ | 
*UsersApi* | [**users_purchase_track_record_create**](docs/UsersApi.md#users_purchase_track_record_create) | **POST** /api2/users/{id}/purchase-track-record/ | 
*UsersApi* | [**users_retrieve**](docs/UsersApi.md#users_retrieve) | **GET** /api2/users/{id}/ | 
*UsersApi* | [**users_unlock_power_create**](docs/UsersApi.md#users_unlock_power_create) | **POST** /api2/users/{id}/unlock-power/ | 
*UsersApi* | [**users_update**](docs/UsersApi.md#users_update) | **PUT** /api2/users/{id}/ | 


## Documentation For Models

 - [Boost](docs/Boost.md)
 - [BulkPredictionInput](docs/BulkPredictionInput.md)
 - [Category](docs/Category.md)
 - [Comment](docs/Comment.md)
 - [CommentChildren](docs/CommentChildren.md)
 - [CommentUpdate](docs/CommentUpdate.md)
 - [Considerations](docs/Considerations.md)
 - [ExtendedPredictionUsername](docs/ExtendedPredictionUsername.md)
 - [Notification](docs/Notification.md)
 - [NotificationTypeEnum](docs/NotificationTypeEnum.md)
 - [Option](docs/Option.md)
 - [Organization](docs/Organization.md)
 - [OrganizationDetail](docs/OrganizationDetail.md)
 - [PaginatedCategoryList](docs/PaginatedCategoryList.md)
 - [PaginatedCommentList](docs/PaginatedCommentList.md)
 - [PaginatedNotificationList](docs/PaginatedNotificationList.md)
 - [PaginatedOrganizationList](docs/PaginatedOrganizationList.md)
 - [PaginatedPredictionUsernameList](docs/PaginatedPredictionUsernameList.md)
 - [PaginatedProjectList](docs/PaginatedProjectList.md)
 - [PaginatedProjectUserStatsList](docs/PaginatedProjectUserStatsList.md)
 - [PaginatedQuestionUserList](docs/PaginatedQuestionUserList.md)
 - [PaginatedRankingList](docs/PaginatedRankingList.md)
 - [PaginatedReminderList](docs/PaginatedReminderList.md)
 - [PaginatedUserList](docs/PaginatedUserList.md)
 - [PaginatedUserProfileList](docs/PaginatedUserProfileList.md)
 - [PatchedCommentUpdate](docs/PatchedCommentUpdate.md)
 - [PatchedOrganization](docs/PatchedOrganization.md)
 - [PatchedProject](docs/PatchedProject.md)
 - [PatchedQuestionUpdate](docs/PatchedQuestionUpdate.md)
 - [PatchedReminder](docs/PatchedReminder.md)
 - [PatchedUser](docs/PatchedUser.md)
 - [PatchedUserProfile](docs/PatchedUserProfile.md)
 - [Prediction](docs/Prediction.md)
 - [PredictionForDate](docs/PredictionForDate.md)
 - [PredictionInput](docs/PredictionInput.md)
 - [PredictionUsername](docs/PredictionUsername.md)
 - [Project](docs/Project.md)
 - [ProjectDetail](docs/ProjectDetail.md)
 - [ProjectOrganization](docs/ProjectOrganization.md)
 - [ProjectUpdate](docs/ProjectUpdate.md)
 - [ProjectUserStats](docs/ProjectUserStats.md)
 - [Question](docs/Question.md)
 - [QuestionProject](docs/QuestionProject.md)
 - [QuestionRelated](docs/QuestionRelated.md)
 - [QuestionResolve](docs/QuestionResolve.md)
 - [QuestionSummary](docs/QuestionSummary.md)
 - [QuestionSummaryFeedback](docs/QuestionSummaryFeedback.md)
 - [QuestionTypes](docs/QuestionTypes.md)
 - [QuestionUpdate](docs/QuestionUpdate.md)
 - [QuestionUpdateStatusEnum](docs/QuestionUpdateStatusEnum.md)
 - [QuestionUser](docs/QuestionUser.md)
 - [QuestionUserDetail](docs/QuestionUserDetail.md)
 - [QuestionVote](docs/QuestionVote.md)
 - [Ranking](docs/Ranking.md)
 - [Reminder](docs/Reminder.md)
 - [ReminderEnum](docs/ReminderEnum.md)
 - [ReminderStatusEnum](docs/ReminderStatusEnum.md)
 - [RepeatPatternEnum](docs/RepeatPatternEnum.md)
 - [ScoreTypeEnum](docs/ScoreTypeEnum.md)
 - [ShareQuestion](docs/ShareQuestion.md)
 - [ShowCommunity](docs/ShowCommunity.md)
 - [Status3baEnum](docs/Status3baEnum.md)
 - [SubQuestionList](docs/SubQuestionList.md)
 - [SubQuestionUpdate](docs/SubQuestionUpdate.md)
 - [SubQuestionUserDetail](docs/SubQuestionUserDetail.md)
 - [SubQuestionUserList](docs/SubQuestionUserList.md)
 - [SubmitTypeEnum](docs/SubmitTypeEnum.md)
 - [User](docs/User.md)
 - [UserCommunityVisEnum](docs/UserCommunityVisEnum.md)
 - [UserFeedback](docs/UserFeedback.md)
 - [UserProfile](docs/UserProfile.md)
 - [ValueEnum](docs/ValueEnum.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

Benjamin Manns <opensource@benmanns.com>

