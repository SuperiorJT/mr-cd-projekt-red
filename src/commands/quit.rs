use crate::ShardManagerContainer;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Shuts down the bot"]
#[usage("~quit")]
#[owners_only]
pub async fn quit(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        manager.lock().await.shutdown_all().await;
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    let _ = msg.reply(&ctx, "Shutting down!");

    Ok(())
}
