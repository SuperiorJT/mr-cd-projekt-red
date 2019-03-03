use std::sync::Arc;

use crate::{audio::Receiver, BufferType, VoiceManager};

use serenity::model::misc::Mentionable;
use serenity::voice;

command!(join(ctx, msg, _args) {
    let channel_id = match msg.guild() {
        Some(guild) => {
            match guild.read().voice_states.get(&msg.author.id) {
                Some(state) => match state.channel_id {
                    Some(id) => id,
                    None => {
                        warn!("User is not in voice channel");

                        return Ok(())
                    }
                },
                None => {
                    warn!("Could not get voice state for user {:?}", msg.author.name);

                    return Ok(())
                }
            }
        },
        None => {
            warn!("Could not fetch guild");

            return Ok(())
        }
    };
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            warn!("Message does not contain GuildId");

            return Ok(())
        }
    };

    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let mut buffer_lock = ctx.data.lock().get::<BufferType>().cloned().expect("Expected BufferType in ShareMap.");

    if let Some(handler) = manager.join(guild_id, channel_id) {
        handler.listen(Some(Box::new(Receiver::new(Arc::clone(&buffer_lock)))));
        if let Err(why) = msg.channel_id.say(&format!("Joined {}", channel_id.mention())) {
            error!("Error sending message: {:?}", why);
        }

        // Currently there is a bug that requires us to play audio when we join
        // If we don't do this, the bot will not play audio over voice later on
        let source = match voice::ffmpeg("audio_static/join.m4a") {
            Ok(source) => source,
            Err(why) => {
                error!("Err starting source: {:?}", why);

                return Ok(());
            },
        };

        handler.play(source);
    } else {
        if let Err(why) = msg.channel_id.say("Error joining the channel") {
            error!("Error sending message: {:?}", why);
        }
    }
});
