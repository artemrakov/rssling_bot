use std::process;

use rssling_bot::db::setup;

#[tokio::main]
async fn main() {
    if let Err(err) = setup::create_tables().await {
        eprintln!("Error: {:?}", err);
        process::exit(1);
    }
}
