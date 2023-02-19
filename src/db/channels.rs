use super::{error::Error, error::Error::MongoQueryError, types::Channel, DB};
use crate::db::{types::RssEntry, DB_NAME};
use bson::{to_bson, to_vec};
use log::info;
use mongodb::{
    bson::{doc, from_document, to_document, Document},
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

    pub async fn find_channel(&self, channel: &Channel) -> Result<Option<Channel>, Error> {
        let channel = self
            .channels()
            .find_one(
                doc! {
                    URL: &channel.url
                },
                None,
            )
            .await?;

        if let None = channel {
            return Ok(None);
        }

        info!("Document found channel: #{:?}", channel);

        let channel = from_document(channel.unwrap());

        Ok(Some(channel.unwrap()))
    }

    pub async fn update_channel(&self, channel: &Channel) -> Result<(), Error> {
        let entries: Vec<&RssEntry> = channel
            .entries
            .iter()
            .filter(|entry| entry.pub_date > channel.updated_at)
            .collect();

        info!(
            "New entries to channel id: #{:?}. Entries: #{:?}",
            channel.id, entries
        );

        let entries_doc = to_bson(&entries).unwrap();
        let updated_channel = self
            .channels()
            .update_one(
                doc! {
                    ID: channel.id.clone().unwrap()
                },
                doc! {
                    "$set": { TITLE: &channel.title },
                    "$currentDate": { UPDATED_AT: true },
                    "$push": { ENTRIES: entries_doc }
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
        if let Some(found_channel) = self.find_channel(&channel).await? {
            self.update_channel(&found_channel).await?;
            return Ok(());
        }

        self.create_channel(&channel).await?;
        Ok(())
    }
}
