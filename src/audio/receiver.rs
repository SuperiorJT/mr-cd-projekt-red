use std::{
    collections::HashMap,
    sync::Arc,
    sync::RwLock,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use config::{Config, File};
use serde::Deserialize;

use serenity::model::id::UserId;
use serenity::voice::AudioReceiver;

use super::buffer::DiscordAudioBuffer;
use super::DiscordAudioPacket;

#[derive(Deserialize)]
struct UserMixConfig {
    volume: f32,
    mute: bool,
}

impl Default for UserMixConfig {
    fn default() -> Self {
        Self {
            volume: 1.0,
            mute: false,
        }
    }
}

pub struct Receiver {
    buffer: Arc<RwLock<DiscordAudioBuffer>>,
    mix_config: HashMap<u64, UserMixConfig>,
}

impl Receiver {
    pub fn new(buffer: Arc<RwLock<DiscordAudioBuffer>>) -> Self {
        let mut config = Config::new();
        if let Err(err) = config.merge(File::with_name("config/mixer.json")) {
            error!("{} - Using empty config", err);
        }
        let mix_config = match config.try_into::<HashMap<u64, UserMixConfig>>() {
            Ok(c) => c,
            Err(_) => HashMap::new(),
        };
        Self { buffer, mix_config }
    }
}

impl AudioReceiver for Receiver {
    fn speaking_update(&mut self, ssrc: u32, user_id: u64, speaking: bool) {
        let user = match UserId::from(user_id).to_user() {
            Ok(user) => user,
            Err(why) => {
                error!("Cannot get user defined in speaking update: {:?}", why);

                return;
            }
        };
        let mut buffer = match self.buffer.write() {
            Ok(buffer) => buffer,
            Err(why) => {
                error!("Could not get audio buffer lock: {:?}", why);

                return;
            }
        };
        let volume = self.mix_config.entry(user_id).or_default().volume;
        buffer.update_track_mix(ssrc, volume);
        info!("Speaking Update: {}, {}, {}", user.name, ssrc, speaking);
    }

    fn voice_packet(
        &mut self,
        ssrc: u32,
        sequence: u16,
        _timestamp: u32,
        stereo: bool,
        data: &[i16],
    ) {
        // info!("Audio packet's first 5 bytes: {:?}", data.get(..5));
        // info!(
        //     "Audio packet sequence {:05} has {:04} bytes, SSRC {}",
        //     sequence,
        //     data.len(),
        //     ssrc,
        // );
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let timestamp = since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_millis() as u64;
        info!(
            "Time: {}, Sequence: {}, ssrc: {}",
            timestamp, sequence, ssrc
        );
        let mut buffer = match self.buffer.write() {
            Ok(buffer) => buffer,
            Err(why) => {
                error!("Could not get audio buffer lock: {:?}", why);

                return;
            }
        };
        buffer.insert_item(DiscordAudioPacket::new(
            ssrc,
            sequence,
            timestamp,
            stereo,
            data.to_owned(),
        ));
        // info!("Data Size: {}, Buffer Length: {}, Buffer Cap: {}", data.len(), buffer.size(), buffer.capacity());
    }
}
