use crate::db::types::Channel;
use std::error::Error;
use url::Url;

pub async fn fetch_channel(url: Url) -> Result<(), Box<dyn Error + Send + Sync>> {
    let body = reqwest::get(url).await?.text().await?;

    Ok(())
}
