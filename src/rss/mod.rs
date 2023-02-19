use super::db::types::{Channel, RssEntry};
use log::info;
use reqwest::Url;
use std::error::Error;

const NUMBER_OF_ITEMS: usize = 10;

pub async fn fetch_channel(url: Url) -> Result<(), Box<dyn Error + Send + Sync>> {
    let content = reqwest::get(url).await?.text().await?;

    let channel = match content.parse::<syndication::Feed>() {
        Ok(syndication::Feed::RSS(rss)) => parse_rss(rss),
        Ok(syndication::Feed::Atom(atom)) => parse_atom(atom),
        _ => Err("Could not parse feed"),
    };

    Ok(())
}

fn parse_atom(atom: atom_syndication::Feed) -> Result<Channel, &'static str> {
    let entries: Vec<RssEntry> = atom
        .entries()
        .iter()
        .take(NUMBER_OF_ITEMS)
        .map(|entry| RssEntry {
            id: None,
            title: entry.title.value,
            pub_date: entry.updated.into(),
            url: entry.id,
            created_at: chrono::offset::Utc::now(),
        })
        .collect();

    let channel = Channel {
        id: None,
        title: atom.title.value,
        url: atom.id,
        updated_at: chrono::offset::Utc::now(),
        entries,
        subs: vec![],
    };

    Ok(channel)
}

fn parse_rss(rss: rss::Channel) -> Result<Channel, &'static str> {
    // let channel = channel.unwrap();
    // info!("Channel: #{:?}", &rss_channel);

    // let entries: Vec<RssEntry> = rss_channel
    //     .items()
    //     .iter()
    //     .take(NUMBER_OF_ITEMS)
    //     .map(|item| RssEntry {
    //         id: None,
    //         title: item.title.clone(),
    //         description: item.description.clone(),
    //         pub_date: item.pub_date.clone(),
    //         url: item.link.clone(),
    //         created_at: chrono::offset::Utc::now(),
    //     })
    //     .collect();
    //
    // let channel = Channel {
    //     id: None,
    //     title: rss_channel.title.clone(),
    //     url: rss_channel.link.clone(),
    //     description: rss_channel.description.clone(),
    //     updated_at: chrono::offset::Utc::now(),
    //     entries,
    //     subs: vec![],
    // };
}
