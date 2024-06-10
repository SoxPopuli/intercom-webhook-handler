use super::{ s, dt };
use std::collections::HashMap;

use crate::domain::{
    conversation::{Conversation, Tag, *},
    notification::Notification,
    DateTime,
};
use chrono::serde::ts_seconds;

use serde::Deserialize;

const CONVERSATION_JSON: &str = include_str!("./data_files/conversation.json");
const NOTIFICATION_JSON: &str = include_str!("./data_files/notification.json");

#[test]
fn notification_test() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Company {
        #[serde(rename = "type")]
        typ: String,
        id: String,
        name: String,
        company_id: String,
        #[serde(deserialize_with = "ts_seconds::deserialize")]
        remote_created_at: DateTime,
        #[serde(deserialize_with = "ts_seconds::deserialize")]
        created_at: DateTime,
        #[serde(deserialize_with = "ts_seconds::deserialize")]
        updated_at: DateTime,
        custom_attributes: HashMap<String, String>,
    }

    let notification: Notification<Company> = serde_json::from_str(NOTIFICATION_JSON).unwrap();

    assert_eq!(
        notification,
        Notification {
            typ: s("notification_event"),
            id: s("notif_ccd8a4d0-f965-11e3-a367-c779cae3e1b3"),
            url: None,
            created_at: dt("2014-02-18T13:48:51Z"),
            topic: s("company.created"),
            delivery_attempts: 1,
            first_sent_at: dt("2014-02-18T13:49:52Z"),
            data: Company {
                typ: s("company"),
                id: s("531ee472cce572a6ec000006"),
                name: s("Blue Sun"),
                company_id: s("6"),
                remote_created_at: dt("2014-03-11T09:46:09Z"),
                created_at: dt("2014-03-11T10:25:06Z"),
                updated_at: dt("2014-04-07T12:44:18Z"),
                custom_attributes: HashMap::new(),
            },
        }
    )
}

#[test]
fn conversation_deserialization_test() {
    let conversation: Conversation =
        serde_json::from_str(CONVERSATION_JSON).expect("Failed to deserialize conversation");

    assert_eq!(
        conversation,
        Conversation {
            typ: s("conversation"),
            id: s("1295"),
            title: Some(s("Conversation Title"),),
            created_at: dt("2022-09-19T14:20:23Z"),
            updated_at: dt("2022-09-19T14:21:00Z"),
            waiting_since: Some(dt("2022-09-19T14:21:00Z"),),
            snoozed_until: Some(dt("2022-09-19T14:21:00Z"),),
            open: true,
            state: ConversationState::Open,
            read: true,
            priority: ConversationPriority::Priority,
            admin_assignee_id: Some(0,),
            team_assignee_id: Some(s("5017691"),),
            tags: vec![Tag {
                typ: s("tag"),
                id: s("123456"),
                name: s("Test tag"),
                applied_at: dt("2022-09-19T14:20:23Z"),
            },],
            conversation_rating: Some(ConversationRating {
                rating: 5,
                remark: s(""),
                created_at: dt("2022-12-14T14:41:34Z"),
                contact: ContactReference {
                    reference: Reference {
                        typ: s("contact"),
                        id: s("5ba682d23d7cf92bef87bfd4"),
                    },
                    external_id: Some(s("f3b87a2e09d514c6c2e79b9a"),),
                },
                teammate: Reference {
                    typ: s("contact"),
                    id: s("1a2b3c"),
                },
            },),
            source: ConversationSource {
                typ: s("conversation"),
                id: s("3"),
                delivered_as: s("operator_initiated"),
                subject: s(""),
                body: s("<p>Hey there!</p>"),
                author: Author {
                    typ: s("admin"),
                    id: s("274"),
                    name: s("Operator"),
                    email: s("operator+abcd1234@intercom.io"),
                },
                attachments: vec![Attachment {
                    typ: s("upload"),
                    name: s("example.png"),
                    url: s("https://picsum.photos/200/300"),
                    content_type: s("image/png"),
                    filesize: 100,
                    width: 100,
                    height: 100,
                },],
                url: None,
                redacted: false,
            },
            contacts: vec![ContactReference {
                reference: Reference {
                    typ: s("contact"),
                    id: s("5ba682d23d7cf92bef87bfd4"),
                },
                external_id: Some(s("f3b87a2e09d514c6c2e79b9a"),),
            },],
            teammates: vec![],
            custom_attributes: [(s("property2"), s("string")), (s("property1"), s("string")),]
                .into_iter()
                .collect(),
            first_contact_reply: Some(FirstContactReply {
                created_at: dt("2022-09-19T14:20:23Z"),
                typ: s("conversation"),
                url: Some(s("https://developers.intercom.com/"),),
            },),
            sla_applied: Some(AppliedSLA {
                typ: s("conversation_sla_summary"),
                sla_name: s(""),
                sla_status: SLAStatus::Hit,
            },),
            statistics: Some(Statistics {
                typ: s("conversation_statistics"),
                time_to_assignment: 2310,
                time_to_admin_reply: 2310,
                time_to_first_close: 2310,
                time_to_last_close: 2310,
                median_time_to_reply: 2310,
                first_contact_reply_at: dt("2022-09-19T14:20:33Z"),
                first_assignment_at: dt("2022-09-19T14:20:33Z"),
                first_admin_reply_at: dt("2022-09-19T14:20:33Z"),
                first_close_at: dt("2022-09-19T14:20:33Z"),
                last_assignment_at: dt("2022-09-19T14:20:33Z"),
                last_assignment_admin_reply_at: dt("2022-09-19T14:20:33Z"),
                last_contact_reply_at: dt("2022-09-19T14:20:33Z"),
                last_admin_reply_at: dt("2022-09-19T14:20:33Z"),
                last_close_at: dt("2022-09-19T14:20:33Z"),
                last_closed_by_id: s("c3po"),
                count_reopens: 1,
                count_assignments: 1,
                count_conversation_parts: 1,
            },),
            ai_agent_participated: true,
            ai_agent: AIAgent {
                source_type: SourceType::Workflow,
                source_title: Some(s("My AI Workflow"),),
                last_answer_type: Some(LastAnswerType::AIAnswer,),
                resolution_state: ResolutionState::AssumedResolution,
                rating: 4,
                rating_remark: s("Very helpful!"),
                content_sources: vec![ContentSources {
                    content_type: ContentType::ContentSnippet,
                    url: s("/fin-ai-agent/content?content=content_snippet&id=3234924"),
                    title: s("My internal content snippet"),
                    locale: s("en"),
                },],
            },
        }
    );

    assert_eq!(conversation.created_at, dt("2022-09-19T14:20:23Z"));

    assert_eq!(
        conversation.tags[0],
        Tag {
            typ: "tag".into(),
            id: "123456".into(),
            name: "Test tag".into(),
            applied_at: dt("2022-09-19T14:20:23Z"),
        }
    );
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
struct TagsContainer {
    #[serde(deserialize_with = "Tag::deserialize_from_tags_wrapper")]
    tags: Vec<Tag>,
}

#[test]
fn tag_list_deserialize() {
    let now = dt("2024-01-01T00:00:00Z");

    let expected = vec![Tag {
        typ: s("tag"),
        id: s("1"),
        name: s("tag name"),
        applied_at: now,
    }];

    let tag_list_json = serde_json::json!({
        "type": "tag.list",
        "tags": [
            {
                "type": "tag",
                "id": "1",
                "name": "tag name",
                "applied_at": 1704067200
            }
        ]
    });

    let tags: TagsContainer = serde_json::from_value(tag_list_json).unwrap();

    assert_eq!(expected, tags.tags)
}
