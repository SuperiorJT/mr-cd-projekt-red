use std::sync::{Arc, RwLock};

use std::collections::HashMap;

use super::buffer::DiscordAudioBuffer;

pub struct BufferManager {
    buffers: HashMap<u64, Arc<RwLock<DiscordAudioBuffer>>>,
}

impl BufferManager {
    fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        guild_id: u64,
        buffer: Arc<RwLock<DiscordAudioBuffer>>,
    ) -> Option<Arc<RwLock<DiscordAudioBuffer>>> {
        self.buffers.insert(guild_id, buffer)
    }

    pub fn get(&self, guild_id: &u64) -> Option<&Arc<RwLock<DiscordAudioBuffer>>> {
        self.buffers.get(guild_id)
    }
}
