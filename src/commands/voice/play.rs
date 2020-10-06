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
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a valid filename").await,
            );

            return Ok(());
        }
    };

    let path_opt = std::fs::read_dir("audio_static")?
        .map(|entry| entry.unwrap())
        .find(|entry| {
            let path = entry.path();
            let file_name = path.file_stem().expect("File name does not exist");
            println!("{:?}", file_name);
            if file_name == std::ffi::OsStr::new(&name) {
                return true;
            }
            false
        });
    let path = match path_opt {
        Some(p) => p,
        None => {
            check_msg(msg.channel_id.say(
                &ctx.http,
                format!("Could not find file with name: {}", name),
            ).await);

            return Ok(());
        }
    };

    let guild_id = match ctx.cache.guild_channel(msg.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info").await);

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

    let handler = match manager.get_mut(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Not in a voice channel to play in").await,
            );

            return Ok(());
        }
    };

    let source = match voice::ffmpeg(path.path()).await {
        Ok(source) => source,
        Err(why) => {
            error!("{}", why);
            for entry in
                std::fs::read_dir(std::env::current_dir().unwrap()).expect("Unable to list")
            {
                let entry = entry.expect("unable to get entry");
                error!("{}", entry.path().display());
            }

            check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

            return Ok(());
        }
    };

    handler.play(source);

    check_msg(msg.channel_id.say(&ctx.http, "Playing audio").await);

    Ok(())
}
