use crate::data;

use serenity::http::Http;
use serenity::model::id::ChannelId;

/*  using gateway
use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
*/

use rand::Rng;

//struct Handler;

fn __lottery() -> &'static str {
    // choose rarity
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..10); // create number from 0-9

    // 0-5(60%): common, 6-8(30%): rare, 9(10%): super-rare
    let mut rarity: &str = "rare";
    if n < 6 {
        rarity = "common";
    } else if n == 9 {
        rarity = "super_rare";
    }

    // for debug
    println!("n: {}, rarity: {}", n, rarity);

    rarity
}

fn lottery(range: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(0..range); // create number from 0-9

    n
}

pub async fn send(token: &str, id_str: &str, common: Vec<&data::Data>, rare: Vec<&data::Data>, super_rare: Vec<&data::Data>) {
    let http = Http::new(token);
    let id_num: u64 = id_str.parse().expect("Failed to parse number");
    let channel_id = ChannelId::new(id_num); // ここに実際のチャンネルIDを設定

    let r_num: i32 = lottery(10);

    // 0-5(60%): common, 6-8(30%): rare, 9(10%): super-rare
    let mut rarity: &str = "rare";
    let mut message_content: &str = "rare";

    if r_num < 6 {
        rarity = "common";
        message_content = "common";
    } else if r_num == 9 {
        rarity = "super_rare";
        message_content = "super_rare";
    }

    // for debug
    println!("r_num: {}, rarity: {}", r_num, rarity);


    // sending message content to channel
    if let Err(why) = channel_id.say(&http, message_content).await {
        println!("Error sending message: {:?}", why);
    }

    /*  using gateway
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("err creating client");

    if let Err(why) = client.start().await {
        println!("client error: {why:?}");
    }
    */
}
