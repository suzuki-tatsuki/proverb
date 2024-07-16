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

fn lottery() -> &'static str {
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

/*
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "proverb" {
            lottery();

            // send message
            if let Err(why) = msg.channel_id.say(&ctx.http, "test").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
*/

pub async fn send(token: &str, common: Vec<&data::Data>, rare: Vec<&data::Data>, super_rare: Vec<&data::Data>) {
    let http = Http::new(token);
    let channel_id = ChannelId::new(123456789012345678); // ここに実際のチャンネルIDを設定

    //let message_content = format!("Common data: {:?}", common::proverb);

    let message_content: &str = lottery();

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
