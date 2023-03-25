use bot::{message_handler, start_bot};
use futures::stream::{self, StreamExt};
use types::RssEntry;
use std::{error::Error, sync::Arc};
use teloxide::{types::{Update, UpdateKind}, Bot};
use tracing::{error, info};
use teloxide::prelude::*;

use crate::{db::DB, rss::fetch_channel};

pub mod bot;
pub mod db;
pub mod rss;
pub mod types;

const PARALLEL_REQUESTS: usize = 3;
const CONCURRENT_REQUESTS: usize = 3;

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn process_bot_message(body: String) -> HandlerResult {
    let update: Update = serde_json::from_str(&body).unwrap();
    let bot = start_bot().await?;

    match update.kind {
        UpdateKind::Message(message) => message_handler(bot, message).await?,
        _ => panic!("Expected `Message`"),
    }

    Ok(())
}

pub async fn fetch_updates_from_feed() -> HandlerResult {
    info!("Fetching update from feeds");
    let db_client = DB::init().await.unwrap();

    let channels = db_client.all_channels().await.unwrap();
    info!("Updating the channels: #{:?}", channels);

    let bodies = stream::iter(channels)
        .map(|channel| {
            let url = channel.url.clone();
            let db_client = db_client.clone();

            tokio::spawn(async move {
                let updated_channel = fetch_channel(url).await.expect("Failed to fetch channel");
                db_client
                    .create_or_update_channel(&updated_channel)
                    .await
                    .expect("Failed to update channel");

                Ok(channel) as reqwest::Result<_>
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(channel)) => info!("Updated channel #{:?}", channel),
                Ok(Err(e)) => error!("Got a reqwest::Error: {}", e),
                Err(e) => error!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;

    Ok(())
}

pub async fn send_notifications() -> HandlerResult {
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

    Ok(())
}

fn format_message(channel_name: &str, entries: Arc<Vec<RssEntry>>) -> String {
    let markdown_urls: Vec<String> = entries
        .iter()
        .map(|entry| format!("<a href='{}'>{}</a>", entry.url, entry.title))
        .collect();

    format!(
        "<strong>{}</strong>\n{}",
        channel_name,
        markdown_urls.join("\n")
    )
}
