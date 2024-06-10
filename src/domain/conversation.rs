use chrono::serde::{ts_seconds, ts_seconds_option};
use paste::paste;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use super::DateTime;

/// Generate a function that extracts an array from an object with a type field
/// and the array of name `$field`
///
/// The name of the generated function is `deserialize_from_<$field>_wrapper`
macro_rules! impl_deserialize_from_wrapper {
    ($typ: ty, $field: ident) => {
        impl $typ {
            paste! {
                pub(crate) fn [<deserialize_from_ $field _wrapper>]<'de, D>(de: D) -> Result<Vec<Self>, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    #[derive(Deserialize)]
                    struct Wrapper {
                        #[serde(rename = "type")]
                        _typ: String,
                        $field: Vec<$typ>,
                    }

                    match Wrapper::deserialize(de) {
                        Ok(wrapper) => Ok(wrapper.$field),
                        Err(_) => Ok(vec![]),
                    }
                }
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tag {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub applied_at: DateTime,
}
impl_deserialize_from_wrapper!(Tag, tags);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Reference {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
}
impl_deserialize_from_wrapper!(Reference, teammates);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContactReference {
    #[serde(flatten)]
    pub reference: Reference,
    pub external_id: Option<String>,
}
impl_deserialize_from_wrapper!(ContactReference, contacts);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationState {
    Open,
    Closed,
    Snoozed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationPriority {
    Priority,
    NotPriority,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversationRating {
    pub rating: i8,
    pub remark: String,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub created_at: DateTime,
    pub contact: ContactReference,
    pub teammate: Reference,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Author {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attachment {
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub url: String,
    pub content_type: String,
    pub filesize: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConversationSource {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
    pub delivered_as: String,
    pub subject: String,
    pub body: String,
    pub author: Author,
    pub attachments: Vec<Attachment>,
    pub url: Option<String>,
    pub redacted: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirstContactReply {
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub created_at: DateTime,
    #[serde(rename = "type")]
    pub typ: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SLAStatus {
    Hit,
    Missed,
    Cancelled,
    Active,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppliedSLA {
    #[serde(rename = "type")]
    pub typ: String,
    pub sla_name: String,
    pub sla_status: SLAStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Statistics {
    #[serde(rename = "type")]
    pub typ: String,
    pub time_to_assignment: u32,
    pub time_to_admin_reply: u32,
    pub time_to_first_close: u32,
    pub time_to_last_close: u32,
    pub median_time_to_reply: u32,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub first_contact_reply_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub first_assignment_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub first_admin_reply_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub first_close_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub last_assignment_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub last_assignment_admin_reply_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub last_contact_reply_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub last_admin_reply_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub last_close_at: DateTime,
    pub last_closed_by_id: String,
    pub count_reopens: i32,
    pub count_assignments: i32,
    pub count_conversation_parts: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    File,
    Article,
    ExternalContent,
    ContentSnippet,
    WorkflowConnectorAction,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentSources {
    pub content_type: ContentType,
    pub url: String,
    pub title: String,
    pub locale: String,
}
impl_deserialize_from_wrapper!(ContentSources, content_sources);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    EssentialsPlanSetup,
    Profile,
    Workflow,
    WorkflowPreview,
    FinPreview,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LastAnswerType {
    #[serde(rename = "ai_answer")]
    AIAnswer,
    CustomAnswer,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionState {
    AssumedResolution,
    ConfirmedResolution,
    RoutedToTeam,
    Abandoned,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AIAgent {
    pub source_type: SourceType,
    pub source_title: Option<String>,
    pub last_answer_type: Option<LastAnswerType>,
    pub resolution_state: ResolutionState,
    pub rating: u8,
    pub rating_remark: String,
    #[serde(deserialize_with = "ContentSources::deserialize_from_content_sources_wrapper")]
    pub content_sources: Vec<ContentSources>,
}

/// [Type definition](https://developers.intercom.com/docs/references/rest-api/api.intercom.io/Conversations/conversation/)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Conversation {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
    pub title: Option<String>,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub created_at: DateTime,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub updated_at: DateTime,
    #[serde(deserialize_with = "ts_seconds_option::deserialize")]
    pub waiting_since: Option<DateTime>,
    #[serde(deserialize_with = "ts_seconds_option::deserialize")]
    pub snoozed_until: Option<DateTime>,
    pub open: bool,
    pub state: ConversationState,
    pub read: bool,
    pub priority: ConversationPriority,
    pub admin_assignee_id: Option<i32>,
    pub team_assignee_id: Option<String>,
    #[serde(deserialize_with = "Tag::deserialize_from_tags_wrapper")]
    pub tags: Vec<Tag>,
    pub conversation_rating: Option<ConversationRating>,
    pub source: ConversationSource,
    #[serde(deserialize_with = "ContactReference::deserialize_from_contacts_wrapper")]
    pub contacts: Vec<ContactReference>,
    #[serde(deserialize_with = "Reference::deserialize_from_teammates_wrapper")]
    pub teammates: Vec<Reference>,
    pub custom_attributes: HashMap<String, String>,
    pub first_contact_reply: Option<FirstContactReply>,
    pub sla_applied: Option<AppliedSLA>,
    pub statistics: Option<Statistics>,
    pub ai_agent_participated: bool,
    pub ai_agent: AIAgent,
}
