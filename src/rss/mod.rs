use super::types::{Channel, RssEntry};
use atom_syndication::Link;
use chrono::{DateTime, Utc};
use tracing::info;
use std::{error::Error, sync::Arc};

pub async fn fetch_channel(url: String) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    let content = reqwest::get(&url).await?.text().await?;

    let channel = match content.parse::<syndication::Feed>() {
        Ok(syndication::Feed::RSS(rss)) => parse_rss(url, rss),
        Ok(syndication::Feed::Atom(atom)) => parse_atom(url, atom),
        _ => Err("Could not parse feed")?,
    };

    info!("Channel: #{:?}", channel);
    channel
}

fn parse_atom(
    url: String,
    atom: atom_syndication::Feed,
) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    info!("Parsing atom: {}", url);
    let entries: Vec<RssEntry> = atom
        .entries()
        .iter()
        .map(|entry| {
            let link = entry
                .links()
                .get(0)
                .unwrap_or(&Link::default())
                .href()
                .to_string();

            RssEntry {
                title: entry.title.value.clone(),
                pub_date: entry.updated.into(),
                url: link,
                created_at: Utc::now(),
            }
        })
        .collect();

    let channel = Channel {
        id: None,
        title: atom.title.value,
        url,
        updated_at: Utc::now(),
        entries: Arc::new(entries),
        subs: vec![],
    };

    Ok(channel)
}

fn parse_rss(
    url: String,
    rss_channel: rss::Channel,
) -> Result<Channel, Box<dyn Error + Send + Sync>> {
    info!("Parsing rss: {}", url);

    let entries: Vec<RssEntry> = rss_channel
        .items()
        .iter()
        .map(|item| {
            let pub_date = item.pub_date.clone().unwrap_or(Utc::now().to_rfc2822());
            let parsed_date = DateTime::parse_from_rfc2822(&pub_date).unwrap();

            RssEntry {
                title: item.title.clone().unwrap_or("".to_string()),
                pub_date: parsed_date.into(),
                url: item.link.clone().unwrap_or("".to_string()),
                created_at: Utc::now(),
            }
        })
        .collect();

    let channel = Channel {
        id: None,
        title: rss_channel.title,
        url,
        updated_at: Utc::now(),
        entries: Arc::new(entries),
        subs: vec![],
    };

    Ok(channel)
}
