use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands};
use url::{ParseError, Url};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Subscribe to rss feed")]
    Sub(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Sub(link) => {
            subscribe_to_rss(msg.from().unwrap().id, &link);

            bot.send_message(msg.chat.id, format!("Success")).await?
        },
    };

    Ok(())
}

fn subscribe_to_rss(user_id: UserId, link: &str) -> Result<(), Box<dyn Error>> {
    let parsed_url = Url::parse(link)?;

    Ok(())
}
