use std::sync::Arc;

use bson::oid::ObjectId;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub mod channel;
pub mod notification;
pub mod subscription;

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(skip_serializing, skip_deserializing)]
    pub entries: Arc<Vec<RssEntry>>,
    pub subs: Vec<Subscription>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub telegram_id: String,
    pub status: SubscriptionStatus,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RssEntry {
    pub title: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub pub_date: DateTime<Utc>,
    pub url: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub telegram_id: String,
    pub channel_name: String,
    pub channel_url: String,
    pub entries: Arc<Vec<RssEntry>>,
    pub sent: bool,
}
