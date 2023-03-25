use aws_lambda_events::encodings::Body;
use aws_lambda_events::{
    apigw::ApiGatewayProxyRequest, apigw::ApiGatewayProxyResponse, http::HeaderMap,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rssling_bot::{fetch_updates_from_feed, process_bot_message, send_notifications};
use tracing::info;

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    info!("Received request: {:?}", &event);
    let path = event.payload.path.as_ref().unwrap();

    match path.as_str() {
        "/default/rssling_bot" => handle_bot_message(&event).await,
        "/fetch_updates_from_feed" => handle_fetch_updates_from_feed().await,
        "/send_notifications" => handle_notifications().await,
        _ => panic!("Unknown path"),
    }
}

async fn handle_notifications() -> Result<ApiGatewayProxyResponse, Error> {
    send_notifications().await?;

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        body: Some(Body::Text("Ok".to_string())),
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}

async fn handle_fetch_updates_from_feed() -> Result<ApiGatewayProxyResponse, Error> {
    fetch_updates_from_feed().await?;

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        body: Some(Body::Text("Ok".to_string())),
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}

async fn handle_bot_message(
    event: &LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = event.payload.body.clone().unwrap();

    process_bot_message(body).await?;

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

// #[test]
// fn test_my_lambda_handler() {
//   let input = include_str!("apigw_proxy_request.json");
//
//   let request = lambda_http::request::from_str(input)
//     .expect("failed to create request");
//
//   let response = my_lambda_handler(request).await.expect("failed to handle request");
// }
