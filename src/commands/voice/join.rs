use crate::{audio::Receiver, commands::helpers::check_msg, BufferType, VoiceManager};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, misc::Mentionable},
    prelude::Context,
};
use std::sync::Arc;

#[command]
#[description = "Joins the requester's voice channel"]
pub fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = match msg.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Groups and DMs not supported"),
            );

            return Ok(());
        }
    };

    let channel_id = guild
        .read()
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_states| voice_states.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(&ctx, "Not in a voice channel"));

            return Ok(());
        }
    };

    let guild_id = guild.read().id;

    let mut manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let mut buffer_lock = ctx
        .data
        .read()
        .get::<BufferType>()
        .cloned()
        .expect("Expected BufferType in ShareMap.");

    let handler = match manager.join(guild_id, connect_to) {
        Some(handler) => handler,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel"));

            return Ok(());
        }
    };

    handler.listen(Some(Box::new(Receiver::new(Arc::clone(&buffer_lock)))));

    check_msg(
        msg.channel_id
            .say(&ctx.http, &format!("Joined {}", connect_to.mention())),
    );

    Ok(())
}

// command!(join(ctx, msg, _args) {
//     if let Some(handler) = manager.join(guild_id, channel_id) {
//         handler.listen(Some(Box::new(Receiver::new(Arc::clone(&buffer_lock)))));
//         if let Err(why) = msg.channel_id.say(&format!("Joined {}", channel_id.mention())) {
//             error!("Error sending message: {:?}", why);
//         }

//         // Currently there is a bug that requires us to play audio when we join
//         // If we don't do this, the bot will not play audio over voice later on
//         let source = match voice::ffmpeg("audio_static/join.m4a") {
//             Ok(source) => source,
//             Err(why) => {
//                 error!("Err starting source: {:?}", why);

//                 return Ok(());
//             },
//         };

//         handler.play(source);
//     } else {
//         if let Err(why) = msg.channel_id.say("Error joining the channel") {
//             error!("Error sending message: {:?}", why);
//         }
//     }
// });
