use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event
    async fn message(&self, ctx: Context, msg: Message) {
        // println!("{}", msg.author.name);

        if msg.author.name != "Dinobot" {
            if let Err(why) = msg.channel_id.say(&ctx.http, msg.content).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[derive(Serialize, Deserialize)]
struct BotConfig {
    discord_token: String,
}

#[tokio::main]
async fn main() {
    let file: File = File::open("config.json").expect("Failed to open config file");

    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Failed to read the config file");

    let config: BotConfig = serde_json::from_str(&contents).expect("Failed to serialize config");

    let token = config.discord_token;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
