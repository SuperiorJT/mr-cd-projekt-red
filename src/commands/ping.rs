use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Pings the bot"]
#[usage("~ping")]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(ctx, "Pong!");

    Ok(())
}
