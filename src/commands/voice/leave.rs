use crate::{commands::helpers::check_msg, VoiceManager};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
#[description = "Leaves the current voice channel"]
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Groups and DMs not supported").await,
            );

            return Ok(());
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .await
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock().await;
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(&ctx.http, "Not in a voice channel").await);
    }

    Ok(())
}
