use aws_lambda_events::encodings::Body;
use aws_lambda_events::{
    apigw::ApiGatewayProxyRequest, apigw::ApiGatewayProxyResponse, http::HeaderMap, serde_json,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rssling_bot::{message_handler, start_bot};
use teloxide::types::{Update, UpdateKind};
use tracing::info;

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Received request: {:?}", event);
    let body = event.payload.body.unwrap();

    let update: Update = serde_json::from_str(&body).unwrap();
    let bot = start_bot().await?;

    match update.kind {
        UpdateKind::Message(message) => message_handler(bot, message).await?,
        _ => panic!("Expected `Message`"),
    }

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        body: Some(Body::Text("Ok".to_string())),
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: Some(false),
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
