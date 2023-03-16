use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event
    async fn message(&self, ctx: Context, msg: Message) {
        let channel = msg.channel_id.to_channel(&ctx.http).await.unwrap();

        let channel_name = channel.guild().unwrap().name;

        if msg.author.name != "Dinobot" && channel_name == "bots" {
            if let Err(why) = msg.channel_id.say(&ctx.http, msg.content).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);
    }
}

#[derive(Serialize, Deserialize)]
struct BotConfig {
    token: String,
}

#[tokio::main]
async fn main() {
    let config_file = std::fs::read_to_string("config.json").unwrap();

    let config: BotConfig = serde_json::from_str(&config_file).expect("Failed to serialize config");

    let token = config.token;

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
