use super::super::super::VoiceManager;

use crate::commands::helpers::check_msg;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
    voice,
};

#[command]
#[description = "Plays the requested audio clip if available"]
#[usage("[filename].[ext]")]
#[aliases("p", "play")]
#[min_args(1)]
pub fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a valid filename"),
            );

            return Ok(());
        }
    };

    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));

            return Ok(());
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Not in a voice channel to play in"),
            );

            return Ok(());
        }
    };

    let source = match voice::ffmpeg(&name) {
        Ok(source) => source,
        Err(why) => {
            error!("{}", why);
            for entry in
                std::fs::read_dir(std::env::current_dir().unwrap()).expect("Unable to list")
            {
                let entry = entry.expect("unable to get entry");
                error!("{}", entry.path().display());
            }

            check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg"));

            return Ok(());
        }
    };

    handler.play(source);

    check_msg(msg.channel_id.say(&ctx.http, "Playing audio"));

    Ok(())
}
