use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
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

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);

        let guild_command =
            Command::create_global_application_command(&ctx.http, |command| register_ping(command))
                .await;

        println!(
            "I created the following global slash command: {:#?}",
            guild_command
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => run_ping(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct BotConfig {
    token: String,
    guild_id: u64,
}

#[tokio::main]
async fn main() {
    let token = get_config().token;

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

pub fn run_ping(_options: &[CommandDataOption]) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register_ping(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}

fn get_config() -> BotConfig {
    let config_file = std::fs::read_to_string("conf.json").unwrap();

    let config: BotConfig = serde_json::from_str(&config_file).expect("Failed to serialize config");

    return config;
}
