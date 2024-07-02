mod data;
mod get;
mod send;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> std::result::Result<(), String> {
    dotenv().expect(".env file not loaded");
    let sheet_id = env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID not found");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found");

    let proverbs = get::get_data(&sheet_id).await?;

    let common = proverbs.iter()
        .filter(|x| x.rarity == data::Rarity::Common)
        .collect::<Vec<&data::Data>>();
    let rare = proverbs.iter()
        .filter(|x| x.rarity == data::Rarity::Rare)
        .collect::<Vec<&data::Data>>();
    let super_rare = proverbs.iter()
        .filter(|x| x.rarity == data::Rarity::SuperRare)
        .collect::<Vec<&data::Data>>();

    send::send(&discord_token).await;
    Ok(())
}
