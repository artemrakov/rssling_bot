use super::{error::Error, error::Error::MongoQueryError, types::User, DB};
use crate::db::DB_NAME;
use log::info;
use mongodb::{
    bson::{doc, Document},
    Collection,
};

const CHANNELS: &str = "channels";

const ID: &str = "_id";
const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
const URL: &str = "url";

impl DB {
    fn channels(&self) -> Collection<Document> {
        let db = self.client.database(DB_NAME);

        db.collection::<Document>(CHANNELS)
    }

}
