#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;

mod audio;
mod commands;
use audio::{DiscordAudioBuffer, DiscordAudioPacket, Receiver};

use std::{collections::HashSet, env, sync::Arc, sync::RwLock};

use serenity::{
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
    framework::standard::help_commands,
    framework::StandardFramework,
    http,
    model::gateway::Ready,
    prelude::*,
};

use typemap::Key;

pub struct VoiceManager;

impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct BufferType;

impl Key for BufferType {
    type Value = Arc<RwLock<DiscordAudioBuffer>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

pub static DISCORD_SAMPLE_RATE: usize = 48000;

pub static TRACK_LENGTH: usize = 5;

pub static PACKETS_PER_SECOND: usize = 50;

pub static BUFFER_LENGTH: usize = PACKETS_PER_SECOND * 3 * 60;

fn main() {
    kankyo::load().expect("Failed to load .env file");
    env_logger::init().expect("Failed to initialize env_logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let buffer_map = Arc::new(RwLock::new(DiscordAudioBuffer::new(BUFFER_LENGTH)));

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.lock();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<BufferType>(Arc::clone(&buffer_map));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .cmd("ping", commands::ping::ping)
            .command("quit", |c| c.cmd(commands::quit::quit).owners_only(true))
            .group("voice", |g| {
                g.command("join", |c| c.known_as("j").cmd(commands::voice::join::join))
                    .command("play", |c| c.known_as("p").cmd(commands::voice::play::play))
                    .command("save", |c| c.known_as("s").cmd(commands::voice::save::save))
            })
            .help(help_commands::with_embeds),
    );

    if let Err(err) = client.start() {
        println!("An Error ocurred while running the client: {:?}", err);
    }
}
