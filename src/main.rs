#[macro_use]
extern crate log;

pub mod audio;
pub mod commands;
pub mod db;
pub mod error;
pub mod model;

pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

use audio::DiscordAudioBuffer;
use db::Database;

use std::{collections::HashSet, env, sync::Arc, sync::RwLock};

use serenity::{
    client::{
        bridge::{gateway::ShardManager, voice::ClientVoiceManager},
        Client, Context, EventHandler,
    },
    framework::{standard::macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::*,
};

use commands::{
    about::*,
    admin::shadow_realm::*,
    db_test::*,
    help::*,
    ping::*,
    quit::*,
    voice::{join::*, leave::*, play::*},
};

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct BufferType;

impl TypeMapKey for BufferType {
    type Value = Arc<RwLock<DiscordAudioBuffer>>;
}

pub struct DBType;

impl TypeMapKey for DBType {
    type Value = Arc<RwLock<Database>>;
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

pub static BUFFER_LENGTH: usize = PACKETS_PER_SECOND * 1 * 60;

#[group]
#[commands(about, ping, quit, register)]
struct General;

#[group]
#[commands(join, leave, play)]
struct Voice;

#[group]
#[commands(shadow_realm)]
struct Admin;

fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Error creating client");

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let db = Arc::new(RwLock::new(
        Database::open().expect("Couldn't open database"),
    ));

    let buffer_map = Arc::new(RwLock::new(DiscordAudioBuffer::new(BUFFER_LENGTH)));

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write();
        data.insert::<DBType>(Arc::clone(&db));
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<BufferType>(Arc::clone(&buffer_map));
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP)
            .group(&VOICE_GROUP)
            .group(&ADMIN_GROUP)
            .help(&CDPR_HELP),
    );

    if let Err(err) = client.start() {
        println!("An Error ocurred while running the client: {:?}", err);
    }
}
