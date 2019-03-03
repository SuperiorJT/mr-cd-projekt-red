use crate::{audio::DiscordAudioBuffer, BufferType, VoiceManager};
use core::f32::consts::PI;

use serenity::CACHE;

static I16_MAX: i64 = i16::max_value() as i64 + 1;

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

    if let Some(handler) = manager.get_mut(guild_id) {
        let audio_data = build_data_map(&buffer, name.clone());
        info!("{}", audio_data.len());
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

fn build_data_map(buffer: &DiscordAudioBuffer, name: String) -> Vec<i16> {
    let mut tracks = buffer
        .data_map
        .values()
        .map(|t| t.build_packets())
        .collect::<Vec<Vec<i16>>>();

    let mut largest_track_size = 0;

    for (ssrc, track) in buffer.data_map.iter() {
        info!("{}, {}", ssrc, track.len());
    }

    tracks.iter().for_each(|track| {
        if track.len() > largest_track_size {
            largest_track_size = track.len();
        }
    });

    // Test some stuff
    // for (ssrc, track) in buffer.data_map.iter() {
    //     write_to_file(track.build_packets(), format!("{}{}", ssrc, name));
    // }
    // End Test

    // Pad tracks that ended before others
    debug!("Largest Track Size: {}", largest_track_size);
    tracks.iter_mut().for_each(|track| {
        while track.len() < largest_track_size {
            track.push(0);
        }
        debug!("track size: {}", track.len());
    });

    let mut data = vec![];

    for i in 0..largest_track_size {
        let mut mix = 0;
        for track in tracks.iter() {
            mix = mix_samples_alt(mix, track[i], tracks.len());
        }
        data.push(mix);
    }

    debug!("Final data size: {}", data.len());

    data
}

fn build_data(buffer: &DiscordAudioBuffer) -> Vec<i16> {
    info!("Buffer size: {}, {}", buffer.size(), buffer.data.len());
    let mut tracked_ssrcs = vec![];
    buffer.data.iter().rev().for_each(|x| {
        if x.ssrc != 0 {
            if !tracked_ssrcs.contains(&x.ssrc) {
                tracked_ssrcs.push(x.ssrc);
            }
        }
    });
    let mut prev_frame = 0;
    let mut prev_ssrc = 0;
    let mut packet_data = vec![];
    let mut result = vec![];
    let (front, back) = buffer.data.as_slices();
    let mut buffer_data = vec![];
    buffer_data.extend_from_slice(front);
    buffer_data.extend_from_slice(back);
    buffer_data.sort_by(|a, b| a.frame.cmp(&b.frame));
    buffer_data.iter().for_each(|x| {
        let mut data_clone = x
            .data
            .iter()
            .map(|y| *y / tracked_ssrcs.len() as i16)
            .collect::<Vec<i16>>();
        if prev_ssrc == 0 {
            packet_data.clear();
            packet_data.append(&mut data_clone);
        // debug!("Clone");
        } else if prev_frame == x.frame {
            packet_data = packet_data
                .iter()
                .enumerate()
                .map(|y| {
                    let z = data_clone[y.0];
                    if y.0 == 0 {
                        info!("Before {} {}", *y.1, z);
                    }
                    let out = mix_samples_alt(*y.1, z, tracked_ssrcs.len());
                    if y.0 == 0 {
                        info!("After {}", out);
                    }
                    if out == 32767 || out == -32768 {
                        info!("CLIPPED {}", out);
                    }
                    out
                })
                .collect();
        // debug!("Mix size: {}", packet_data.len());
        } else {
            // debug!("Save {}", packet_data.len());
            result.append(&mut packet_data);
            // debug!("Result size: {}", result.len());
            packet_data.clear();
            packet_data.append(&mut data_clone);
            // debug!("Clone");
        }
        prev_frame = x.frame;
        prev_ssrc = x.ssrc;
    });
    result.append(&mut packet_data);
    // buffer.data.iter().rev().for_each(|x| {
    //     result.append(&mut x.data.clone());
    // });
    debug!(
        "before: {}, after: {}",
        buffer.data.len() * 1920,
        result.len()
    );
    result
}

fn mix_samples(a: i16, b: i16) -> i16 {
    let mut out: i64;
    let ua = a as i64 + I16_MAX;
    let ub = b as i64 + I16_MAX;
    if ua < I16_MAX && ub < I16_MAX {
        out = ua * ub / I16_MAX;
    } else {
        out = 2 * (ua + ub) - (ua * ub) / I16_MAX - (I16_MAX * 2);
    }
    if out >= I16_MAX * 2 {
        info!("CLIPPING");
        out = I16_MAX * 2 - 1;
    }
    (out - I16_MAX) as i16
}

fn mix_samples_alt(a: i16, b: i16, ssrc_count: usize) -> i16 {
    let na = normalize_sample(a);
    let nb = normalize_sample(b);
    // debug!("{} {}", na, nb);
    let res = na + nb;
    // debug!("add {}", res);
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
    // for t in (0..48000).map(|x| x as f32 / 48000.0) {
    //     let sample = (t * 440.0 * 2.0 * PI).sin();
    //     let amplitude = I16_MAX as f32;
    //     writer.write_sample((sample * amplitude) as i16).unwrap();
    // }
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
