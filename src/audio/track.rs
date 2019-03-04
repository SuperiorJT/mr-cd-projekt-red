use super::discord_audio_packet::{PACKET_INTERVAL, PACKET_SIZE};
use super::DiscordAudioPacket;
use std::collections::VecDeque;

pub struct Track {
    data: VecDeque<DiscordAudioPacket>,
    pub prev_packet: Option<DiscordAudioPacket>,
    pub volume: f32, // 0 to 1 inclusive
}

impl Track {
    pub fn new(volume: f32) -> Self {
        Self {
            data: VecDeque::new(),
            prev_packet: None,
            volume,
        }
    }

    pub fn new_with_items(volume: f32, items: Vec<DiscordAudioPacket>) -> Self {
        let mut data = VecDeque::new();
        for p in items {
            data.push_front(p);
        }
        let prev_packet = Some(data.front().unwrap().clone());
        Self {
            data,
            prev_packet,
            volume,
        }
    }

    pub fn insert_packet(&mut self, data: DiscordAudioPacket) {
        let mut front: DiscordAudioPacket = match self.prev_packet.take() {
            Some(p) => p,
            None => {
                self.prev_packet = Some(data.clone());
                self.data.push_front(data);
                return;
            }
        };

        if front.sequence > data.sequence {
            error!("Ignoring packet received out of order");
            self.prev_packet = Some(front);
            return;
        }
        if data.sequence != front.sequence {
            let dropped_packets = data.sequence - front.sequence - 1;
            if dropped_packets > 0 {
                debug!("DROPPED PACKETS");
                for x in 0..dropped_packets {
                    self.data.push_front(DiscordAudioPacket::new(
                        data.ssrc,
                        front.sequence + (x + 1),
                        front.timestamp + ((x as u64 + 1) * PACKET_INTERVAL),
                        data.stereo,
                        vec![0; PACKET_SIZE],
                    ));
                }
                front = self.data.front().unwrap().clone();
            }
        }
        if data.timestamp > front.timestamp {
            let silence_time = data.timestamp - front.timestamp;
            if silence_time > 60 {
                let silence_frames =
                    (silence_time as f32 / PACKET_INTERVAL as f32).round() as u64 - 1;

                for x in 0..silence_frames {
                    self.data.push_front(DiscordAudioPacket::new(
                        data.ssrc,
                        data.sequence,
                        front.timestamp + ((x + 1) * PACKET_INTERVAL),
                        data.stereo,
                        vec![0; PACKET_SIZE],
                    ));
                }
            }
        }

        self.prev_packet = Some(data.clone());

        self.data.push_front(data);
    }

    pub fn pop(&mut self) {
        self.data.pop_back();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn build_packets(&self) -> Vec<i16> {
        let (front, back) = self.data.as_slices();
        let mut buffer_data = vec![];
        buffer_data.extend_from_slice(front);
        buffer_data.extend_from_slice(back);
        buffer_data.sort_by(|a, b| a.sequence.cmp(&b.sequence));

        buffer_data
            .iter()
            .map(|p| {
                p.data
                    .clone()
                    .iter_mut()
                    .map(|data| (*data as f32 * self.volume) as i16)
                    .collect::<Vec<i16>>()
            })
            .flatten()
            .collect()
    }
}
