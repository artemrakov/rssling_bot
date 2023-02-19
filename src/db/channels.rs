use super::{error::Error, error::Error::MongoQueryError, types::Channel, types::User, DB};
use crate::db::DB_NAME;
use log::info;
use mongodb::{
    bson::{doc, Document, to_document},
    Collection,
};
use bson::DateTime;


const CHANNELS: &str = "channels";

const ID: &str = "_id";
const URL: &str = "URL";

impl DB {
    fn channels(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(CHANNELS)
    }

    pub async fn find_channel(&self, channel: &Channel) -> Result<Option<Document>, Error> {
        let channel = self
            .channels()
            .find_one(
                doc! {
                    URL: &channel.url
                },
                None,
            )
            .await?;

        Ok(channel)
    }

    pub async fn create_channel(&self, channel: &Channel) -> Result<(), Error> {
        let doc = to_document(channel).unwrap();

        let created_channel = self
            .channels()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        info!("Channel created! #{:?}", created_channel);

        Ok(())
    }

    pub async fn create_or_update_channel(&self, channel: &Channel) -> Result<(), Error> {
        if let Some(_) = self.find_channel(&channel).await? {
            // self.update_channel(&channel).await?;
            return Ok(());
        }

        self.create_channel(&channel).await?;
        Ok(())
    }
}
