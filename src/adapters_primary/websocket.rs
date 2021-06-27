use bastion::run;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{thread, time};

use crate::{
    config,
    domain::{dto::Fighters, fight},
};

struct Handler {
    target_channel_id: String,
    fight_context: fight::Context,
}

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, _: Context, msg: Message) {
        if !msg.author.bot && msg.channel_id.as_u64().to_string() == self.target_channel_id {
            println!("{}: {}", msg.author.name, msg.content);

            if !msg.mentions.is_empty() {
                let fighters = Fighters {
                    fighter1: msg.author.name.clone(),
                    fighter2: msg.mentions[0].name.clone(),
                };

                run!(async {
                    if let Err(e) = self.fight_context.resolve_fight(fighters).await {
                        eprintln!("couldn't resolve_fight : {}", e);
                    }
                });
            }
        }
    }
}

pub fn start_websocket(fight_context: fight::Context) {
    let mut client = Client::new(
        config::CONFIG.token.to_string(),
        Handler {
            target_channel_id: config::CONFIG.target_channel_id.to_owned(),
            fight_context,
        },
    )
    .expect("Error while creating client");

    let five_minutes = time::Duration::from_millis(300000);

    loop {
        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
        thread::sleep(five_minutes);
    }
}
