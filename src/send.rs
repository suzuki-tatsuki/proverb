use serenity::async_trait;
use serenity::model::gateway::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;

use std::env;

struct Handler;

const TOKEN_NAME: &str = "DISCORD_TOKEN";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "School::school_content" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "school_message").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn connector() {
    let token = env::var(TOKEN_NAME)
        .expect("expected a token in the environment");

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
}
 
pub fn send() {
    connector();
}
