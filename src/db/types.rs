use bson::oid::ObjectId;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub telegram_id: String,
    pub first_name: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub url: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub entries: Vec<RssEntry>,
    pub subs: Vec<Subscription>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub user_id: String,
    pub status: SubscriptionStatus,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    pub latest_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssEntry {
    pub title: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub pub_date: DateTime<Utc>,
    pub url: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Read {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub seen: bool,
}
