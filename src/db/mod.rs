use self::error::Error;
use log::info;
use mongodb::{bson::doc, options::ClientOptions, Client};

pub mod error;
pub mod types;
pub mod users;
pub mod channels;

const DB_NAME: &str = "rssling_bot";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some(DB_NAME.to_string());
        let client = Client::with_options(client_options)?;
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        info!("Connected successfully.");

        Ok(Self { client })
    }
}
