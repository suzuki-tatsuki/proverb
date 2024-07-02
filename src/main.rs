mod data;
mod get;
mod send;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not loaded");
    let sheet_id = env::var("SPREAD_SHEET_ID").expect("SPREAD_SHEET_ID not found");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found");

    let data = get::get_data(&sheet_id).await;

    send::send(&discord_token).await;
}
