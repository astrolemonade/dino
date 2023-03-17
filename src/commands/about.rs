use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run_about(_options: &[CommandDataOption]) -> String {
    return "I am Dino Bot".to_string();
}

pub fn register_about(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    return command.name("about").description("what is Dinobot?");
}
