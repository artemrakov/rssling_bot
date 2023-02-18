use chrono::DateTime;
use chrono::Utc;
use url::Url;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub first_name: String,
    pub username: String,
}
#[derive(Debug)]
pub struct Channel {
    pub id: Option<String>,
    pub name: String,
    pub url: Url,
    pub updated: DateTime<Utc>,
    pub urls: Vec<RssEntry>,
    pub subs: Vec<Subscription>,
}

#[derive(Debug)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
}

#[derive(Debug)]
pub struct Subscription {
    pub id: String,
    pub user_id: String,
    pub status: SubscriptionStatus,
    pub created_at: DateTime<Utc>,
    pub latest_url: Url,
}

#[derive(Debug)]
pub struct RssEntry {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub pub_date: Option<String>,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Read {
    pub id: String,
    pub user_id: String,
    pub seen: bool,
}
