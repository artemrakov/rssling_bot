use super::{error::Error, error::Error::MongoQueryError, types::User, DB};
use crate::db::DB_NAME;
use log::info;
use mongodb::{
    bson::{doc, Document, to_document},
    Collection,
};

const USERS: &str = "users";
const TELEGRAM_ID: &str = "_id";

impl DB {
    fn users(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(USERS)
    }

    pub async fn create_user_if_not_exist(&self, user: &User) -> Result<(), Error> {
        if let Some(_) = self.find_user(&user).await? {
            return Ok(());
        }
        let doc = to_document(&user).unwrap();

        let created_user = self
            .users()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        info!("User created! #{:?}", created_user);

        Ok(())
    }

    pub async fn find_user(&self, user: &User) -> Result<Option<Document>, Error> {
        let user = self
            .users()
            .find_one(
                doc! {
                    TELEGRAM_ID: &user.id.as_ref().unwrap()
                },
                None,
            )
            .await?;

        Ok(user)
    }
}
