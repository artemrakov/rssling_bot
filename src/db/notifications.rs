use super::{error::Error, error::Error::MongoQueryError, DB};
use crate::{db::DB_NAME, types::Notification};
use log::info;
use mongodb::{
    bson::{to_document, Document},
    Collection,
};

const NOTIFICATIONS: &str = "notifications";

// const ID: &str = "_id";
// const TELEGRAM_ID: &str = "telegram_id";
// const CHANNEL_URL: &str = "channel_url";
// const ENTRIES: &str = "entries";
// const SENT: &str = "sent";

impl DB {
    fn notifications(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(NOTIFICATIONS)
    }

    pub async fn create_notifications(
        &self,
        notifications: &Vec<Notification>,
    ) -> Result<(), Error> {
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
