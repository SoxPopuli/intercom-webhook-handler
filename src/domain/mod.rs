pub mod conversation;
pub mod notification;

/// NOTE: Intercom provides times as epoch time in seconds
/// https://www.intercom.com/help/en/articles/3605703-how-dates-work-in-intercom
pub type DateTime = chrono::DateTime<chrono::Utc>;
