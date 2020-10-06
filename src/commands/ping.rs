use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Pings the bot"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, "Pong!");

    Ok(())
}
