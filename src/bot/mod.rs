use crate::{db::DB, HandlerResult};
use crate::{fetch_channel, send_notifications};
use crate::types::User;
use mongodb::bson::doc;
use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use tracing::info;
use url::Url;

const BOT_NAME: &str = "RsstlingBot";

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    Start,
    #[command(description = "Subscribe to rss feed")]
    Sub(String),
}

pub async fn start_bot() -> Result<Bot, Box<dyn Error + Send + Sync>> {
    info!("Starting the bot");
    let bot = Bot::from_env();

    info!("Connecting to the database");
    DB::init().await.unwrap();
    Ok(bot)
}

pub async fn message_handler(bot: Bot, msg: Message) -> HandlerResult {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, BOT_NAME) {
            Ok(Command::Start) => start(&msg, &bot).await?,
            Ok(Command::Sub(link)) => subscribe_to_rss(&msg, &link, &bot).await?,
            Err(_) => {
                bot.send_message(msg.chat.id, "Unknown command".to_string())
                    .await?;
            }
        }
    }

    Ok(())
}

async fn start(msg: &Message, bot: &Bot) -> HandlerResult {
    let db_client = DB::init().await.unwrap();
    let telegram_user = msg.from().unwrap();

    let user = User {
        id: None,
        telegram_id: telegram_user.id.0.to_string(),
        first_name: telegram_user.first_name.clone(),
        username: telegram_user.username.as_ref().unwrap().clone(),
    };

    db_client.create_user_if_not_exist(&user).await?;

    bot.send_message(msg.chat.id, "Hello!").await?;
    Ok(())
}

async fn subscribe_to_rss(msg: &Message, link: &str, bot: &Bot) -> HandlerResult {
    let telegram_id = msg.from().unwrap().id.0.to_string();

    let url = Url::parse(link)?;
    let channel = fetch_channel(url.to_string()).await?;
    let db_client = DB::init().await.unwrap();
    db_client.create_or_update_channel(&channel).await?;

    let found_channel = db_client
        .find_channel(doc! {
            "url": link,
            "subs.telegram_id": &telegram_id,
        })
        .await?;

    if found_channel.is_some() {
        bot.send_message(msg.chat.id, "You have already subscribed to this feed")
            .await?;

        return Ok(());
    }

    db_client
        .subscribe_to_channel(&channel, &telegram_id)
        .await?;

    bot.send_message(msg.chat.id, "Succefully subscribed to the feed")
        .await?;

    send_notifications().await?;
    Ok(())
}
