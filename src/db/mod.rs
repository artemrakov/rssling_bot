use log::info;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};

use self::{error::Error, error::Error::MongoQueryError, types::User};

pub mod error;
pub mod types;

const DB_NAME: &str = "rssling_bot";
const COLL: &str = "users";

const ID: &str = "_id";
const FIRST_NAME: &str = "first_name";
const TELEGRAM_ID: &str = "telegram_id";
const USERNAME: &str = "username";

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
        println!("Connected successfully.");

        Ok(Self { client })
    }

    fn get_collection(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(COLL)
    }

    pub async fn create_user_if_not_exist(&self, user: &User) -> Result<(), Error> {
        if let Some(_) = self.find_user(&user).await? {
            return Ok(());
        }

        let doc = doc! {
            FIRST_NAME: &user.first_name,
            USERNAME: &user.username,
            TELEGRAM_ID: &user.telegram_id,
        };

        let created_user = self
            .get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        info!("User created! #{:?}", created_user);

        Ok(())
    }

    pub async fn find_user(&self, user: &User) -> Result<Option<Document>, Error> {
        let user = self
            .get_collection()
            .find_one(
                doc! {
                    "telegram_id": &user.telegram_id
                },
                None,
            )
            .await?;

        Ok(user)
    }
}
