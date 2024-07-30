use crate::data::{self, Data};

use serenity::all::{CreateMessage, CreateEmbed};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::model::colour::Colour;

use rand::Rng;

fn lottery(range: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..range); // create number from 0-9

    num
}

pub async fn send(token: &str, id_str: &str, common: Vec<&data::Data>, rare: Vec<&data::Data>, super_rare: Vec<&data::Data>) {
    let http = Http::new(token);
    let id_num: u64 = id_str.parse().expect("Failed to parse number");
    let channel_id = ChannelId::new(id_num);

    let r_num: usize = lottery(100);

    // 0-79(80%): common, 80-95(15%): rare, 96-100(5%): super-rare
    let content: &Data;
    let embed_color: Colour;

    if r_num < 80 {
        // common
        let len: usize = common.len();
        let index: usize = lottery(len);
        content = &common[index];
        embed_color = Colour(0x9f7c5c);
    } else if 95 < r_num {
        // super rare
        let len: usize = super_rare.len();
        let index: usize = lottery(len);
        content = &super_rare[index];
        embed_color = Colour(0xffb000);
    } else {
        // rare
        let len: usize = rare.len();
        let index: usize = lottery(len);
        content = &rare[index];
        embed_color = Colour(0xc0c0c0);
    }

    let cnt_speaker = format!("{}", content.speaker);
    let cnt_proverb = format!("# 「{}」", content.proverb);
    let cnt_date = format!("{}", content.date);
    let cnt_rarity = format!("{}", content.rarity);

    if let Err(why) = channel_id.send_message(&http,
        CreateMessage::new()
            .content(cnt_proverb)
            .embed(
                CreateEmbed::new()
                    .color(embed_color)
                    .title(cnt_speaker)
                    .field("レア度", cnt_rarity, true)
                    .field("日付", cnt_date, true)
            )
    ).await {
        println!("Error sending message: {:?}", why);
    }
}
