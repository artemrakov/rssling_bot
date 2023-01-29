use chrono::DateTime;
use chrono::Utc;
use url::Url;

#[derive(Debug, Clone)]
pub struct User {
    /// User unique ID, Telegram generated
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Debug)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub url: Url,
    pub updated: DateTime<Utc>,
}

#[derive(Debug)]
pub enum SubscriptionStatus {
    Active,
    Cancelled
}

#[derive(Debug)]
pub struct Subscriptions  {
    pub name: String,
    pub user_id: i64,
    pub channel_id: i64,
    pub status: SubscriptionStatus,
    pub updated: DateTime<Utc>,
    pub latest_url: Url
}

