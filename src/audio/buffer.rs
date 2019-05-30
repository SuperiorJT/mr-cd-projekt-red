use super::discord_audio_packet::PACKET_SIZE;
use super::DiscordAudioPacket;
use super::Track;
use std::collections::HashMap;

pub struct DiscordAudioBuffer {
    pub data: HashMap<u32, Track>,
    capacity: usize,
    size: usize,
}

impl DiscordAudioBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
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
        if !self.data.contains_key(&data.ssrc) {
            // let mut tracks_to_remove = vec![];
            // let capacity = self.capacity as u64;
            // self.data.iter_mut().for_each(|(ssrc, track)| {
            //     if let Some(p) = track.prev_packet.as_ref() {
            //         if data.timestamp > p.timestamp {
            //             let time_diff = data.timestamp - p.timestamp;
            //             if time_diff > capacity / 50 * 1000 {
            //                 tracks_to_remove.push(*ssrc);
            //             } else if time_diff > 60 {
            //                 track.insert_packet(DiscordAudioPacket::new(
            //                     p.ssrc,
            //                     p.sequence,
            //                     data.timestamp,
            //                     p.stereo,
            //                     vec![0; PACKET_SIZE],
            //                 ));
            //             }
            //         }
            //     }
            // });
            // tracks_to_remove.drain(..).for_each(|ssrc| {
            //     self.data.remove(&ssrc);
            // });
            largest_track_size = self.largest_track_size();
        }
        let track = self.data.entry(data.ssrc).or_insert_with(|| {
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
        // if track_size > self.capacity {
        //     let overflow = track_size - self.capacity;
        //     debug!("OVERFLOW: {}", overflow);
        //     for _ in 0..overflow {
        //         debug!("POP");
        //         self.data.values_mut().for_each(|track| {
        //             track.pop();
        //             debug!("{}", track.len());
        //         });
        //     }
        //     let mut tracks_to_remove = vec![];
        //     self.data.iter().for_each(|(ssrc, track)| {
        //         if track.len() == 0 {
        //             tracks_to_remove.push(*ssrc);
        //         }
        //     });
        //     tracks_to_remove.drain(..).for_each(|ssrc| {
        //         self.data.remove(&ssrc);
        //     });
        // }
    }

    pub fn update_track_mix(&mut self, ssrc: u32, volume: f32) -> Result<(), &'static str> {
        if !self.data.contains_key(&ssrc) {
            return Err("No track for ssrc");
        }

        self.data
            .entry(ssrc)
            .and_modify(|track| track.volume = volume);

        Ok(())
    }

    pub fn largest_track_size(&self) -> usize {
        let mut max = 0;
        for (_ssrc, buffer) in self.data.iter() {
            if buffer.len() > max {
                max = buffer.len();
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {

    use super::DiscordAudioBuffer;
    use super::DiscordAudioPacket;
    use super::PACKET_SIZE;

    #[test]
    fn limits_size_properly() {
        let mut buffer = DiscordAudioBuffer::new(5);

        let mut u1_packets = (0..100).map(|x| new_empty_packet(1, x, (x * 20).into()));
        let mut u2_packets = (0..100).map(|x| new_empty_packet(2, x, (x * 20).into()));
        let mut u3_packets = (0..100).map(|x| new_empty_packet(3, x, (x * 20).into()));

        (0..5).for_each(|x| {
            buffer.insert_item(u1_packets.next().unwrap());
            println!("{}", x);
            buffer.insert_item(u2_packets.next().unwrap());
            println!("{}", x);
            buffer.insert_item(u3_packets.next().unwrap());
            println!("{}", x);
        });

        assert_eq!(buffer.data.len(), 3);
        buffer.data.values().for_each(|t| assert_eq!(t.len(), 5));

        (5..10).for_each(|x| {
            buffer.insert_item(u1_packets.next().unwrap());
            println!("{}", x);
            u2_packets.next();
            u3_packets.next();
        });

        assert_eq!(buffer.data.len(), 1);

        buffer.insert_item(u1_packets.next().unwrap());
        buffer.insert_item(u2_packets.next().unwrap());
        u3_packets.next();

        assert_eq!(buffer.data.len(), 2);
        buffer.data.values().for_each(|t| assert_eq!(t.len(), 5));
    }

    fn new_empty_packet(ssrc: u32, sequence: u16, timestamp: u64) -> DiscordAudioPacket {
        DiscordAudioPacket::new(ssrc, sequence, timestamp, true, vec![0; PACKET_SIZE])
    }
}
