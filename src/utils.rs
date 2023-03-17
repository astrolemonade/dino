// use crate::Command;
use crate::BotConfig;
use serenity::client::Context;
use serenity::model::prelude::GuildId;

pub fn get_config() -> BotConfig {
    let config_file = std::fs::read_to_string("conf.json").unwrap();

    let config: BotConfig = serde_json::from_str(&config_file).expect("Failed to serialize config");

    return config;
}

#[allow(dead_code)]
pub async fn delete_all_commands(ctx: &Context) {
    // Get a list of all global commands
    let commands = ctx
        .http
        .get_global_application_commands()
        .await
        .expect("Failed to get global commands");

    // Delete all global commands
    for command in commands {
        ctx.http
            .delete_global_application_command(*command.id.as_u64())
            .await
            .expect("Failed to delete command");
    }

    println!("Deleted all global commands");

    let guild_id: GuildId = GuildId(get_config().guild_id); // replace with your guild ID

    // Get a list of all guild commands
    let commands = ctx
        .http
        .get_guild_application_commands(guild_id.0)
        .await
        .expect("Failed to get guild commands");

    // Delete all guild commands
    for command in commands {
        ctx.http
            .delete_guild_application_command(guild_id.0, *command.id.as_u64())
            .await
            .expect("Failed to delete command");
    }

    println!("Deleted all guild commands for guild {}", guild_id);
}
