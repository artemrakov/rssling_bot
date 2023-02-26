use lambda_http::{
    aws_lambda_events::serde_json, run, service_fn, Body, Error, Request, Response,
};
use rssling_bot::{message_handler, start_bot};
use teloxide::requests::Requester;
use tracing::info;

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    info!("Request: {:?}", request);

    let bot = start_bot().await?;
    let me = bot.get_me().await?;

    let body = request.body();
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
