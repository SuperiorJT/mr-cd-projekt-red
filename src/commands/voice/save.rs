use crate::{audio::DiscordAudioBuffer, BufferType, VoiceManager};

use serenity::CACHE;

command!(save(ctx, msg, args) {
    let name = match args.single::<String>() {
        Ok(mut name) => {
            if !name.ends_with(".wav") {
                name.push_str(".wav");
            }
            name
        },
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
    let mut buffer_lock = ctx.data.lock().get::<BufferType>().cloned().expect("Expected BufferMap in ShareMap.");
    let mut buffer = buffer_lock.read().expect("Expected buffer to not be poisoned.");

    if let Some(_handler) = manager.get_mut(guild_id) {
        let audio_data = build_data(&buffer);
        write_to_file(audio_data, name);

        if let Err(why) = msg.channel_id.say("Saving audio") {
            error!("Error sending message: {:?}", why);
        }
    } else {
        if let Err(why) = msg.channel_id.say("Not in a voice channel to record in") {
            error!("Error sending message: {:?}", why);
        }
    }
});

fn build_data(buffer: &DiscordAudioBuffer) -> Vec<i16> {
    let mut tracks = buffer
        .data_map
        .values()
        .map(|t| t.build_packets())
        .collect::<Vec<Vec<i16>>>();

    let mut largest_track_size = 0;

    tracks.iter().for_each(|track| {
        if track.len() > largest_track_size {
            largest_track_size = track.len();
        }
    });

    // Pad tracks that ended before others
    tracks.iter_mut().for_each(|track| {
        while track.len() < largest_track_size {
            track.push(0);
        }
    });

    let mut data = vec![];

    for i in 0..largest_track_size {
        let mut mix = 0;
        for track in tracks.iter() {
            mix = mix_samples(mix, track[i]);
        }
        data.push(mix);
    }

    data
}

fn mix_samples(a: i16, b: i16) -> i16 {
    let na = normalize_sample(a);
    let nb = normalize_sample(b);
    let res = na + nb;
    normal_to_i16(res)
}

fn write_to_file(audio_data: Vec<i16>, file_name: String) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer =
        hound::WavWriter::create(file_name, spec).expect("Expected WavWriter to be created.");
    audio_data.iter().for_each(|x| {
        writer
            .write_sample(*x)
            .expect("Expected WavWriter to write sample")
    });
    writer.finalize().expect("Expected WavWriter to finalize.");
}

fn normalize_sample(a: i16) -> f32 {
    if a >= 0 {
        // debug!("{} {}", a, (a as f32 / 32767.0_f32));
        return a as f32 / 32767.0_f32;
    }
    a as f32 / 32768.0_f32
}

fn normal_to_i16(a: f32) -> i16 {
    if a >= 0.0 {
        return (a * 32767.0_f32) as i16;
    }
    (a * 32768.0_f32) as i16
}
