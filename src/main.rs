use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing::info;
use rssling_bot::{message_handler, start_bot};
use serde::Serialize;
use teloxide::types::{Update, UpdateKind};

#[derive(Serialize)]
struct Response {
    msg: String,
}

async fn function_handler(event: LambdaEvent<Update>) -> Result<Response, Error> {
    info!("Received request: {:?}", event);

    let bot = start_bot().await?;
    let update: Update = event.payload;

    match update.kind {
        UpdateKind::Message(message) => message_handler(bot, message).await?,
        _ => panic!("Expected `Message`"),
    }

    let resp = Response {
        msg: "Suceess executed.".to_string(),
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
