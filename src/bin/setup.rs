use std::process;

use rssling_bot::db::DB;

#[tokio::main]
async fn main() {
    if let Err(err) = DB::init().await {
        eprintln!("Error: {err:?}");
        process::exit(1);
    }
}
