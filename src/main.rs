use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response, aws_lambda_events::serde_json};
use rssling_bot::{start_bot, message_handler};
use teloxide::requests::Requester;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let bot = start_bot().await?;
    let me = bot.get_me().await?;

    let body = event.body();
    let message = serde_json::from_slice(body.as_ref())?;

    message_handler(bot, message, me).await?;

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Success".into())
        .map_err(Box::new)?;
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
