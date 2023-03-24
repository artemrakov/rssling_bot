use futures::stream::{self, StreamExt};
use log::{error, info, LevelFilter};
use rssling_bot::{db::DB, rss::fetch_channel};
use simple_logger::SimpleLogger;

const PARALLEL_REQUESTS: usize = 3;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    info!("Fetching update from feeds");
    let db_client = DB::init().await.unwrap();

    let channels = db_client.all_channels().await.unwrap();
    info!("Updating the channels: #{:?}", channels);

    let bodies = stream::iter(channels)
        .map(|channel| {
            let url = channel.url.clone();
            let db_client = db_client.clone();

            tokio::spawn(async move {
                let updated_channel = fetch_channel(url).await.expect("Failed to fetch channel");
                db_client
                    .create_or_update_channel(&updated_channel)
                    .await
                    .expect("Failed to update channel");

                Ok(channel) as reqwest::Result<_>
            })
        })
        .buffer_unordered(PARALLEL_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(Ok(channel)) => info!("Updated channel #{:?}", channel),
                Ok(Err(e)) => error!("Got a reqwest::Error: {}", e),
                Err(e) => error!("Got a tokio::JoinError: {}", e),
            }
        })
        .await;
}
