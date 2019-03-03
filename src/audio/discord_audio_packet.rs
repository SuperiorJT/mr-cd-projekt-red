pub static PACKET_INTERVAL: u64 = 20;
pub static PACKET_SIZE: usize = 1920;

#[derive(Clone)]
pub struct DiscordAudioPacket {
    pub ssrc: u32,
    pub sequence: u16,
    pub timestamp: u64,
    pub frame: u64,
    pub stereo: bool,
    pub data: Vec<i16>,
}

impl DiscordAudioPacket {
    pub fn new(
        ssrc: u32,
        sequence: u16,
        timestamp: u64,
        frame: u64,
        stereo: bool,
        data: Vec<i16>,
    ) -> Self {
        Self {
            ssrc,
            sequence,
            timestamp,
            frame,
            stereo,
            data,
        }
    }
}
