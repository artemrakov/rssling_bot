use super::{error::Error, error::Error::MongoQueryError, types::Channel, types::User, DB};
use crate::db::{types::RssEntry, DB_NAME};
use bson::{DateTime, to_vec};
use log::info;
use mongodb::{
    bson::{doc, to_document, Document},
    Collection,
};

const CHANNELS: &str = "channels";

const ID: &str = "_id";
const TITLE: &str = "title";
const URL: &str = "url";
const UPDATED_AT: &str = "updated_at";
const ENTRIES: &str = "entries";

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

    pub async fn update_channel(&self, channel: &Channel) -> Result<(), Error> {
        let entries: Vec<&RssEntry> = channel
            .entries
            .iter()
            .filter(|entry| entry.pub_date > channel.updated_at)
            .collect();

        let updated_channel = self
            .channels()
            .update_one(
                doc! {
                    ID: channel.id.clone().unwrap()
                },
                doc! {
                    "$set": { TITLE: &channel.title },
                    "$currentDate": { UPDATED_AT: true },
                    "$push": { ENTRIES: to_document(&entries).unwrap() }
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        info!(
            "Update Channel #{:?} with entries: #{:?}",
            updated_channel, &entries
        );

        Ok(())
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
            self.update_channel(&channel).await?;
            return Ok(());
        }

        self.create_channel(&channel).await?;
        Ok(())
    }
}
