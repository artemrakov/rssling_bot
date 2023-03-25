use super::{error::Error, error::Error::MongoQueryError, DB};
use crate::{db::DB_NAME, types::Notification};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, to_document, Document},
    Collection,
};
use tracing::info;

const NOTIFICATIONS: &str = "notifications";

const ID: &str = "_id";
// const TELEGRAM_ID: &str = "telegram_id";
// const CHANNEL_URL: &str = "channel_url";
// const ENTRIES: &str = "entries";
// const SENT: &str = "sent";

impl DB {
    fn notifications(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(NOTIFICATIONS)
    }

    pub async fn all_notifications(&self) -> Result<Vec<Notification>, Error> {
        let mut cursor = self
            .notifications()
            .find(doc! { "sent": false }, None)
            .await?;

        let mut notifications = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            let notification = from_document(doc).unwrap();
            notifications.push(notification);
        }

        Ok(notifications)
    }

    pub async fn update_notification(&self, id: &str) -> Result<(), Error> {
        info!("Updating notification id #{:?}", id);

        let updated_notification = self
            .notifications()
            .update_one(
                doc! {
                    ID: id,
                },
                doc! {
                    "$set": { "sent": true },
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        info!("Updated updated_notification #{:?}", updated_notification);

        Ok(())
    }

    pub async fn create_notifications(
        &self,
        notifications: &Vec<Notification>,
    ) -> Result<(), Error> {
        info!("Creating notifications: #{:?}", notifications);

        let docs: Vec<Document> = notifications
            .iter()
            .map(|notification| to_document(notification).unwrap())
            .collect();

        let create_notifications = self
            .notifications()
            .insert_many(docs, None)
            .await
            .map_err(MongoQueryError)?;

        info!("Notifications created! #{:?}", create_notifications);

        Ok(())
    }
}
