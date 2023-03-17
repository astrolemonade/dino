use crate::Handler;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn dispatch(_sel: &Handler, ctx: Context, msg: Message) {
    let channel = msg.channel_id.to_channel(&ctx.http).await.unwrap();

    let channel_name = channel.guild().unwrap().name;

    if msg.author.name != "Dinobot" {
        if channel_name == "bots" {
            bots_channel(ctx, msg).await;
        } else {
        }
    }
}

async fn bots_channel(ctx: Context, msg: Message) {
    let my_message = "That's cool, tell me more";

    if let Err(why) = msg.channel_id.say(&ctx.http, my_message).await {
        println!("Error sending message: {:?}", why);
    }
}
