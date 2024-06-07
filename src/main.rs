mod error;

use anyhow::{bail, Result};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use opentelemetry_otlp::WithExportConfig;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
#[tracing::instrument]
async fn function_handler(event: Request) -> Result<Response<Body>> {
    let body = match event.body() {
        Body::Text(text) => text,
        Body::Binary(_) => bail!("Recieved binary payload"),
        Body::Empty => bail!("Recieved no payload"),
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("hi".into())?;
    Ok(resp)
}

fn setup_telemetry() {
    let exporter_endpoint = 
        std::env::var("OTEL_ENDPOINT")
        .expect("OTEL_ENDPOINT not set");

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(exporter_endpoint);
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
