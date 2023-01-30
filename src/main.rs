use log::{info, LevelFilter};
use rssling_bot::db::{types::User, DB};
use rssling_bot::rss;
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
                subscribe_to_rss(&msg, &link).await?;

                bot.send_message(msg.chat.id, format!("Success")).await?;
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

async fn subscribe_to_rss(msg: &Message, link: &str) -> HandlerResult {
    let url = Url::parse(link)?;

    rss::fetch_channel(url).await?;

    Ok(())
}
