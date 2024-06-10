#[cfg(test)]
mod tests;

mod domain;
mod error;
mod telemetry;
mod utils;

use anyhow::{bail, Result};
use aws_lambda_events::sqs::SqsEvent;
use aws_sdk_s3::Client as S3Client;
use domain::{conversation::Conversation, notification::Notification, DateTime};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use telemetry::setup_telemetry;
use utils::Pipe;
use uuid::Uuid;

fn get_file_name(now: &DateTime, topic_name: &str, uuid: &Uuid) -> String {
    let topic = topic_name.replace('.', "_");
    let timestamp = now.format("%Y%m%d");

    format!("{timestamp}_{topic}_{uuid}.json")
}

fn generate_file_name(topic_name: &str) -> String {
    let now = chrono::Utc::now();
    let uuid = Uuid::new_v4();
    get_file_name(&now, topic_name, &uuid)
}

async fn push_to_bucket(
    client: &S3Client,
    bucket_name: &str,
    notification: &Notification<Conversation>,
) -> Result<()> {
    let key = generate_file_name(&notification.topic);
    let content = serde_json::to_vec(notification)?;

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(content.into())
        .send()
        .await?;

    Ok(())
}

fn deserialize_conversation(content: &str) -> Option<Notification<Conversation>> {
    let notification = serde_json::from_str::<Notification<Conversation>>(&content);
    match notification {
        Ok(notification) => Some(notification),
        Err(e) => {
            tracing::error!(error = e.to_string(), "record deserialization error");
            None
        }
    }
}

fn log_errors<O, E>(msg: &str, results: &[Result<O, E>])
where
    E: std::fmt::Display,
{
    for result in results {
        if let Err(e) = result {
            tracing::error!(error = e.to_string(), "{}", msg)
        }
    }
}

#[tracing::instrument]
async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<()> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::v2024_03_28()).await;

    let s3_client = aws_sdk_s3::Client::new(&config);

    let bucket_name = std::env::var("OUTPUT_BUCKET")?;

    let records: Vec<_> = event
        .payload
        .records
        .into_iter()
        .filter_map(|x| x.body)
        .filter_map(|msg| deserialize_conversation(&msg))
        .collect();

    let tasks = records
        .iter()
        .map(|r| push_to_bucket(&s3_client, &bucket_name, r));

    let task_results = futures::future::join_all(tasks).await;
    log_errors("error pushing to bucket", &task_results);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _telemetry = if std::env::var("DISABLE_TELEMETRY") != Ok("1".into()) {
        setup_telemetry().await?.pipe(Some)
    } else {
        None
    };

    run(service_fn(function_handler)).await
}
