mod data;
mod get;
mod send;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> std::result::Result<(), String> {
    dotenv().expect(".env file not loaded");
    let sheet_id = env::var("SPREAD_SHEET_ID").expect("SPREAD_SHEET_ID not found");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found");

    let data = get::get_data(&sheet_id).await?;

    let common = data.iter()
        .filter(|x| x.rarity == data::Rarity::Common)
        .collect::<Vec<&data::Data>>();
    let rare = data.iter()
        .filter(|x| x.rarity == data::Rarity::Rare)
        .collect::<Vec<&data::Data>>();
    let super_rare = data.iter()
        .filter(|x| x.rarity == data::Rarity::SuperRare)
        .collect::<Vec<&data::Data>>();

    send::send();

    Ok(())
}
