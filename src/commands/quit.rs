use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Shuts down the bot"]
#[usage("~quit")]
#[owners_only]
pub fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    ctx.quit();

    let _ = msg.reply(ctx, "Shutting down!");

    Ok(())
}
