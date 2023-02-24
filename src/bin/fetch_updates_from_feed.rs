use log::{info, LevelFilter};
use rssling_bot::db::DB;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .expect("Failed to initialize logger");

    info!("Fetching update from feeds");
    let db_client = DB::init().await.unwrap();

    let channels = db_client.all_channels().await;
    info!("Channels: #{:?}", channels);


    // bot.send_message()
}
