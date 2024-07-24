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

    let cnt_speaker = format!("{}", content.speaker);
    let cnt_proverb = format!("# 「{}」", content.proverb);
    let cnt_date = format!("{}", content.date);
    let cnt_rarity = format!("{}", content.rarity);
    //let message_content = format!("{}\n{}「{}」({})", cnt_rarity, cnt_speaker, cnt_proverb, cnt_date);

    // for console
    //println!("r_num: {}, rarity: {}", r_num, rarity);

    if let Err(why) = channel_id.send_message(&http,
        CreateMessage::new()
            .content(cnt_proverb)
            .embed(
                CreateEmbed::new()
                    .color(Colour(0xffffff))
                    //.title(cnt_speaker)
                    .field("レア度", cnt_rarity, true)
                    .field("発言者", cnt_speaker, true)
                    .field("日付", cnt_date, true)
            )
    ).await {
        println!("Error sending message: {:?}", why);
    }
}
