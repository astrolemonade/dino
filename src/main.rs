use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::Member;
use serenity::prelude::*;

mod commands;
mod message;
mod utils;

pub struct Handler;

#[derive(Serialize, Deserialize)]
pub struct BotConfig {
    token: String,
    guild_id: u64,
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event
    async fn message(&self, ctx: Context, msg: Message) {
        message::dispatch::dispatch(self, ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);

        // utils::delete_all_commands(&ctx).await;

        Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register_ping(command)
        })
        .await
        .unwrap();

        Command::create_global_application_command(&ctx.http, |command| {
            commands::about::register_about(command)
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = commands::dispatch::dispatch(&command);

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

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let message = format!("Welcome to the server, {}!", new_member.user.mention());

        let channel_name = "general";

        if let Some(channel) = ctx
            .http
            .get_guild(utils::get_config().guild_id)
            .await
            .unwrap()
            .channels(&ctx.http)
            .await
            .unwrap()
            .values()
            .find(|c| c.name == channel_name)
        {
            if let Err(why) = channel.say(&ctx.http, message).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = utils::get_config().token;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
