use super::super::super::VoiceManager;

use serenity::{voice, CACHE};

command!(play(ctx, msg, args) {
    let name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            if let Err(why) = msg.channel_id.say("Must provide a valid filename") {
                error!("Error sending message: {:?}", why);  
            }

            return Ok(());
        },
    };

    let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            if let Err(why) = msg.channel_id.say("Error finding channel info") {
                error!("Error sending message: {:?}", why);  
            }

            return Ok(());
        },
    };

    let mut manager_lock = ctx.data.lock().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ffmpeg(&name) {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                if let Err(why) = msg.channel_id.say("Error sourcing ffmpeg") {
                    error!("Error sending message: {:?}", why);
                }

                return Ok(());
            },
        };

        handler.play(source);

        if let Err(why) = msg.channel_id.say("Playing audio") {
            error!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg.channel_id.say("Not in a voice channel to play in") {
            error!("Error sending message: {:?}", why);
        }
    }
});
