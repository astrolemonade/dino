use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn dispatch(command: &ApplicationCommandInteraction) -> String {
    let content = match command.data.name.as_str() {
        "ping" => crate::commands::ping::run_ping(&command.data.options),
        "about" => crate::commands::about::run_about(&command.data.options),
        _ => "not implemented :(".to_string(),
    };

    return content;
}
