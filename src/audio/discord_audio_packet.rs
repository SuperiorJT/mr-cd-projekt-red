pub static PACKET_INTERVAL: u8 = 20;
pub static PACKET_SIZE: u16 = 1920;

#[derive(Clone)]
pub struct DiscordAudioPacket {
    pub ssrc: u32,
    pub sequence: u16,
    pub timestamp: u32,
    pub stereo: bool,
    pub data: Vec<i16>,
}

impl DiscordAudioPacket {
    pub fn new(ssrc: u32, sequence: u16, timestamp: u32, stereo: bool, data: Vec<i16>) -> Self {
        Self {
            ssrc,
            sequence,
            timestamp,
            stereo,
            data,
        }
    }
}
