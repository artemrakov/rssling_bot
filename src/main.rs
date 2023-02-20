use log::{info, LevelFilter};
use mongodb::bson::doc;
use rssling_bot::rss;
use rssling_bot::{db::DB, types::User};
use simple_logger::SimpleLogger;
use std::error::Error;
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};
use url::Url;

type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

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

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    info!("starting");

    let bot = Bot::from_env();

    DB::init().await.unwrap();

    let handler = dptree::entry().branch(Update::filter_message().endpoint(message_handler));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> HandlerResult {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Start) => start(&msg).await?,
            Ok(Command::Sub(link)) => {
                info!("Link of sub: {}", &link);

                let message = subscribe_to_rss(&msg, &link).await?;
                bot.send_message(msg.chat.id, message).await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, format!("Unknown command"))
                    .await?;
            }
        }
    }

    Ok(())
}

async fn start(msg: &Message) -> HandlerResult {
    let db_client = DB::init().await.unwrap();
    let telegram_user = msg.from().unwrap();

    let user = User {
        id: None,
        telegram_id: telegram_user.id.0.to_string(),
        first_name: telegram_user.first_name.clone(),
        username: telegram_user.username.as_ref().unwrap().clone(),
    };

    db_client.create_user_if_not_exist(&user).await?;

    Ok(())
}

async fn subscribe_to_rss(
    msg: &Message,
    link: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let telegram_id = msg.from().unwrap().id.0.to_string();

    let url = Url::parse(link)?;
    let channel = rss::fetch_channel(url.to_string()).await?;
    let db_client = DB::init().await.unwrap();
    db_client.create_or_update_channel(&channel).await?;

    let found_channel = db_client
        .find_channel(doc! {
            "url": link,
            "subs.telegram_id": &telegram_id,
        })
        .await?;
    if let Some(_) = found_channel {
        return Ok("You have already subscribed to this feed".to_string());
    }

    db_client
        .subscribe_to_channel(&channel, &telegram_id)
        .await?;

    Ok(format!("Successfully subscribed"))
}
