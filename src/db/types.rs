use chrono::DateTime;
use chrono::Utc;
use url::Url;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub telegram_id: String,
    pub first_name: String,
    pub username: String,
}

#[derive(Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub url: Url,
    pub updated: DateTime<Utc>,
}

#[derive(Debug)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
}

#[derive(Debug)]
pub struct Subscriptions {
    pub id: String,
    pub name: String,
    pub user_id: String,
    pub channel_id: String,
    pub status: SubscriptionStatus,
    pub updated: DateTime<Utc>,
    pub latest_url: Url,
}

#[derive(Debug)]
pub struct RssUrl {
    pub id: String,
    pub url: Url,
    pub channel_id: i64,
}

#[derive(Debug)]
pub struct Read {
    pub id: String,
    pub rss_url_id: String,
    pub user_id: String,
    pub channel_id: String,
    pub seen: bool,
}
