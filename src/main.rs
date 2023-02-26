use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use log::info;
use rssling_bot::{message_handler, start_bot};
use serde::{Deserialize, Serialize};
use teloxide::requests::Requester;

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    info!("Received request: {:?}", event);

    let bot = start_bot().await?;
    let me = bot.get_me().await?;

    let body = event;
    // let message = serde_json::from_slice(body.as_ref())?;

    // message_handler(bot, message, me).await?;
    let resp = Response {
        msg: format!("Suceess executed."),
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

