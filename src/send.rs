use crate::data::{self, Data};

use serenity::http::Http;
use serenity::model::id::ChannelId;

use rand::Rng;

fn lottery(range: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..range); // create number from 0-9

    num
}

pub async fn send(token: &str, id_str: &str, common: Vec<&data::Data>, rare: Vec<&data::Data>, super_rare: Vec<&data::Data>) {
    let http = Http::new(token);
    let id_num: u64 = id_str.parse().expect("Failed to parse number");
    let channel_id = ChannelId::new(id_num); // ここに実際のチャンネルIDを設定

    let r_num: usize = lottery(10);

    // 0-5(60%): common, 6-8(30%): rare, 9(10%): super-rare
    let mut rarity: &str = "rare";
    let content: &Data;

    if r_num < 6 {
        rarity = "common";  // for console

        let len: usize = common.len();
        let index: usize = lottery(len);
        content = &common[index];
    } else if r_num == 9 {
        rarity = "super_rare";  // for cosole

        let len: usize = super_rare.len();
        let index: usize = lottery(len);
        content = &super_rare[index];
    } else {
        // rarity = rare;   // for console

        let len: usize = rare.len();
        let index: usize = lottery(len);
        content = &rare[index];
    }

    let cnt_speaker = &content.speaker;
    let cnt_proverb = &content.proverb;
    let cnt_rarity = &content.rarity;
    let message_content = format!("### {}\n{} 「{}」", cnt_rarity, cnt_speaker, cnt_proverb);

    // for console
    println!("r_num: {}, rarity: {}", r_num, rarity);


    // sending message content to channel
    if let Err(why) = channel_id.say(&http, message_content).await {
        println!("Error sending message: {:?}", why);
    }
}
