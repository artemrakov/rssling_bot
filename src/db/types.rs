use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub first_name: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub url: String,
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
    pub created_at: DateTime<Utc>,
    pub latest_url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RssEntry {
    pub title: String,
    pub pub_date: DateTime<Utc>,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Read {
    pub id: String,
    pub user_id: String,
    pub channel_id: String,
    pub seen: bool,
}
