use super::discord_audio_packet::PACKET_SIZE;
use super::DiscordAudioPacket;
use super::Track;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct DiscordAudioBuffer {
    pub data: VecDeque<DiscordAudioPacket>,
    pub data_map: HashMap<u32, Track>,
    capacity: usize,
    size: usize,
}

impl DiscordAudioBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::new(),
            data_map: HashMap::new(),
            capacity,
            size: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn insert_item(&mut self, data: DiscordAudioPacket) {
        let mut largest_track_size = 0;
        if !self.data_map.contains_key(&data.ssrc) {
            let mut tracks_to_remove = vec![];
            let capacity = self.capacity as u64;
            self.data_map.iter_mut().for_each(|(ssrc, track)| {
                if let Some(p) = track.prev_packet.as_ref() {
                    if data.timestamp > p.timestamp {
                        let time_diff = data.timestamp - p.timestamp;
                        if time_diff > capacity / 50 * 1000 {
                            tracks_to_remove.push(*ssrc);
                        } else if time_diff > 60 {
                            track.insert_packet(DiscordAudioPacket::new(
                                p.ssrc,
                                p.sequence,
                                data.timestamp,
                                0,
                                p.stereo,
                                vec![0; PACKET_SIZE],
                            ));
                        }
                    }
                }
            });
            tracks_to_remove.drain(..).for_each(|ssrc| {
                self.data_map.remove(&ssrc);
            });
            largest_track_size = self.largest_track_size();
        }
        let track = self.data_map.entry(data.ssrc).or_insert_with(|| {
            if largest_track_size == 0 {
                return Track::new(1.0);
            }
            let mut buffer = vec![];
            let len = largest_track_size as u16 - 1;
            for x in 0..len {
                let mut sequence = 0;
                if data.sequence > len {
                    sequence = data.sequence - (len - x);
                }
                buffer.push(DiscordAudioPacket::new(
                    data.ssrc,
                    sequence,
                    data.timestamp - ((len - x) as u64 * 20),
                    0,
                    data.stereo,
                    vec![0; PACKET_SIZE],
                ));
            }
            return Track::new_with_items(1.0, buffer);
        });

        track.insert_packet(data);

        let track_size = track.len();

        debug!("TRACK_SIZE: {}", track_size);

        // Cap our buffer so it doesn't go above capacity
        if track_size > self.capacity {
            let overflow = track_size - self.capacity;
            debug!("OVERFLOW: {}", overflow);
            for _ in 0..overflow {
                debug!("POP");
                self.data_map.values_mut().for_each(|track| {
                    track.pop();
                    debug!("{}", track.len());
                });
            }
            let mut tracks_to_remove = vec![];
            self.data_map.iter().for_each(|(ssrc, track)| {
                if track.len() == 0 {
                    tracks_to_remove.push(*ssrc);
                }
            });
            tracks_to_remove.drain(..).for_each(|ssrc| {
                self.data_map.remove(&ssrc);
            });
        }
    }

    pub fn update_track_mix(&mut self, ssrc: u32, volume: f32) -> Result<(), &'static str> {
        if !self.data_map.contains_key(&ssrc) {
            return Err("No track for ssrc");
        }

        self.data_map
            .entry(ssrc)
            .and_modify(|track| track.volume = volume);

        Ok(())
    }

    pub fn largest_track_size(&self) -> usize {
        let mut max = 0;
        for (_ssrc, buffer) in self.data_map.iter() {
            if buffer.len() > max {
                max = buffer.len();
            }
        }
        max
    }
}
