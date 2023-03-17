use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run_ping(_options: &[CommandDataOption]) -> String {
    return "Hey, I'm alive!".to_string();
}

pub fn register_ping(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    return command.name("ping").description("play ping pong");
}
