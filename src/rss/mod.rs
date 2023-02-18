use super::db::types::{Channel, RssEntry};
use rss::Channel as RssChannel;
use std::error::Error;
use url::Url;

const NUMBER_OF_ITEMS: usize = 10;

pub async fn fetch_channel(url: Url) -> Result<(), Box<dyn Error + Send + Sync>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = RssChannel::read_from(&content[..])?;

    let entries: Vec<RssEntry> = channel
        .items()
        .iter()
        .take(NUMBER_OF_ITEMS)
        .map(|item| RssEntry {
            id: None,
            title: item.title.clone(),
            description: item.description.clone(),
            pub_date: item.pub_date.clone(),
            url: item.link.clone(),
            created_at: chrono::offset::Utc::now(),
        }).collect();

    Ok(())
}
