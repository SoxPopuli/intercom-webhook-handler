mod error;
mod telemetry;
mod utils;

use anyhow::{bail, Result};
use lambda_http::{
    run, service_fn,
    tracing::{self},
    Body, Error, Request, Response,
};
use telemetry::setup_telemetry;
use utils::Pipe;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _telemetry = if std::env::var("DISABLE_TELEMETRY") != Ok("1".into()) {
        setup_telemetry().await?.pipe(Some)
    } else {
        None
    };

    run(service_fn(function_handler)).await
}
