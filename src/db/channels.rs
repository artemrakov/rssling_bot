use std::sync::Arc;

use super::{
    error::Error::{self, MongoQueryError},
    DB,
};
use crate::{
    db::DB_NAME,
    types::{Channel, Subscription},
};
use futures::TryStreamExt;
use tracing::info;
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    Collection,
};

const CHANNELS: &str = "channels";

const ID: &str = "_id";
const TITLE: &str = "title";
const URL: &str = "url";
const UPDATED_AT: &str = "updated_at";
const SUBS: &str = "subs";

impl DB {
    fn channels(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);
        db.collection::<Document>(CHANNELS)
    }

    pub async fn all_channels(&self) -> Result<Vec<Channel>, Error> {
        let mut cursor = self.channels().find(None, None).await?;
        let mut channels = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            let channel = from_document(doc).unwrap();
            channels.push(channel);
        }

        Ok(channels)
    }

    pub async fn find_channel(&self, query: Document) -> Result<Option<Channel>, Error> {
        let channel = self.channels().find_one(query, None).await?;

        if channel.is_none() {
            return Ok(None);
        }

        info!("Document found channel: #{:?}", channel);

        let channel = from_document(channel.unwrap());

        Ok(Some(channel.unwrap()))
    }

    pub async fn subscribe_to_channel(
        &self,
        channel: &Channel,
        telegram_id: &str,
    ) -> Result<(), Error> {
        let subscription = Subscription::new(telegram_id);
        let doc_sub = to_document(&subscription).unwrap();

        self.channels()
            .update_one(
                doc! {
                    URL: channel.url()
                },
                doc! {
                    "$push": { SUBS: doc_sub }
                },
                None,
            )
            .await?;

        info!(
            "Added subscription to #{:?}, sub: #{:?}",
            channel, subscription
        );

        let notification = channel.latest_notification(&subscription);
        if !notification.empty() {
            self.create_notifications(&vec![notification]).await?;
        }

        Ok(())
    }

    pub async fn update_channel(&self, channel: &Channel) -> Result<(), Error> {
        info!("Updating channel: #{:?}", channel);
        let released_entries = Arc::new(channel.released_entries());

        info!(
            "New entries to channel id: #{:?}. Entries: #{:?}",
            channel.url(),
            released_entries
        );

        let active_subs = channel.active_subscriptions();
        if released_entries.len() > 0 && active_subs.len() > 0 {
            let notifications = channel.notifications(active_subs, &released_entries);
            self.create_notifications(&notifications).await?;
        }

        let updated_channel = self
            .channels()
            .update_one(
                doc! {
                    ID: channel.id.unwrap(),
                },
                doc! {
                    "$set": { TITLE: &channel.title },
                    "$currentDate": { UPDATED_AT: true },
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        info!("Updated Channel #{:?}", updated_channel);

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
        if let Some(mut found_channel) = self.find_channel(doc! { URL: &channel.url }).await? {
            found_channel.update_entries(&channel.entries);

            self.update_channel(&found_channel).await?;
            return Ok(());
        }

        self.create_channel(channel).await?;
        Ok(())
    }
}
