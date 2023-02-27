use std::env;

use self::error::Error;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use tracing::info;

pub mod channels;
pub mod error;
pub mod notifications;
pub mod users;

const DB_NAME: &str = "rssling_bot";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let client_uri =
            env::var("MONGO_URI").expect("You must set the MONGODB_URI environment var!");
        let client_options =
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
                .await?;
        let client = Client::with_options(client_options)?;

        info!("Connected successfully.");
        Ok(Self { client })
    }
}
