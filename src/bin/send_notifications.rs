use std::sync::Arc;

use futures::{stream, StreamExt};
use log::{error, info, LevelFilter};
use rssling_bot::{db::DB, types::RssEntry};
use simple_logger::SimpleLogger;
use teloxide::prelude::*;

const CONCURRENT_REQUESTS: usize = 3;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    info!("Starting sending notifications");

    let bot = Bot::from_env();
    let db_client = DB::init().await.unwrap();

    let notifications = db_client.all_notifications().await.unwrap();

    let bodies = stream::iter(notifications)
        .map(|notification| {
            let id = notification.id.unwrap().to_string();
            let bot = bot.clone();
            let telegram_id = notification.telegram_id().to_string();
            let db_client = db_client.clone();
            let message = format_message(&notification.channel_name, notification.entries);
            info!("Message: #{:?}", message);

            async move {
                bot.send_message(telegram_id, message)
                    .disable_web_page_preview(true)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .await
                    .expect("Failed to send message");

                db_client
                    .update_notification(&id)
                    .await
                    .expect("Failed to update notification");

                Ok(id) as reqwest::Result<_>
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(id) => info!("Updated notification #{:?}", id),
                Err(e) => error!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}

fn format_message(channel_name: &str, entries: Arc<Vec<RssEntry>>) -> String {
    let markdown_urls: Vec<String> = entries
        .iter()
        .map(|entry| format!("<a href='{}'>{}</a>", entry.url, entry.title))
        .collect();

    format!("<strong>{}</strong>\n{}", channel_name, markdown_urls.join("\n"))
}
