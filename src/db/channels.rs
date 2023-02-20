use super::{
    error::Error::{self, MongoQueryError},
    DB,
};
use crate::{
    db::DB_NAME,
    types::{Channel, Notification, RssEntry, Subscription, SubscriptionStatus},
};
use chrono::Utc;
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
const SUBS: &str = "subs";
const ENTRIES: &str = "entries";

impl DB {
    fn channels(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(CHANNELS)
    }

    pub async fn find_channel(&self, query: Document) -> Result<Option<Channel>, Error> {
        let channel = self.channels().find_one(query, None).await?;

        if let None = channel {
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
                    URL: channel.url.clone()
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
        let new_entries = channel.new_entries();

        info!(
            "New entries to channel id: #{:?}. Entries: #{:?}",
            channel.id, new_entries
        );

        if new_entries.len() > 0 {
            let active_subs = channel.active_subscriptions();
            let notifications = channel.notifications(active_subs, new_entries);

            self.create_notifications(&notifications).await?;
        }

        let updated_channel = self
            .channels()
            .update_one(
                doc! {
                    ID: channel.id.clone().unwrap()
                },
                doc! {
                    "$set": { TITLE: &channel.title },
                    "$currentDate": { UPDATED_AT: true },
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        info!("Update Channel #{:?}", updated_channel);

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
        if let Some(found_channel) = self.find_channel(doc! { URL: &channel.url }).await? {
            self.update_channel(&found_channel).await?;
            return Ok(());
        }

        self.create_channel(&channel).await?;
        Ok(())
    }
}
