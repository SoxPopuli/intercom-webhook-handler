use super::DateTime;
use chrono::serde::ts_seconds;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

#[derive(Deserialize)]
struct Data<T> {
    pub item: T,
}
impl<'de, T> Data<T>
where
    T: Deserialize<'de>,
{
    fn deserialize_item<D>(de: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = Self::deserialize(de)?;
        Ok(data.item)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Notification<T: DeserializeOwned> {
    #[serde(rename = "type")]
    pub typ: String,
    pub id: String,
    #[serde(rename = "self")]
    pub url: Option<String>,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub created_at: DateTime,
    pub topic: String,
    pub delivery_attempts: i32,
    #[serde(deserialize_with = "ts_seconds::deserialize")]
    pub first_sent_at: DateTime,
    #[serde(deserialize_with = "Data::deserialize_item")]
    pub data: T,
}
