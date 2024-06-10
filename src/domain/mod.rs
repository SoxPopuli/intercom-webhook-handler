pub mod conversation;
pub mod notification;

//NOTE: Intercom provides times as epoch time in seconds
pub type DateTime = chrono::DateTime<chrono::Utc>;
