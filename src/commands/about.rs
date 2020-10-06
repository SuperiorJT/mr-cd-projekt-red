use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Provides a description about the bot"]
pub async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(
        &ctx.http,
        "This is Mr. CD Projekt Red. He will do things in the near future.",
    ).await {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}
