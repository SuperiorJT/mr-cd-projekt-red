#[macro_use]
extern crate log;

// pub mod audio;
pub mod commands;
pub mod db;
pub mod handler;
pub mod error;
pub mod model;

pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;

// use audio::DiscordAudioBuffer;
use db::sqlite::Database;
use db::redis::Redis;

use handler::Handler;

use std::{collections::HashSet, env, sync::Arc, sync::RwLock};

use serenity::{
    http::Http,
    client::{
        Client,
        bridge::{gateway::ShardManager, voice::ClientVoiceManager}
    },
    framework::{standard::macros::group, StandardFramework},
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

// pub struct BufferType;

// impl TypeMapKey for BufferType {
//     type Value = Arc<RwLock<DiscordAudioBuffer>>;
// }

pub struct DBType;

impl TypeMapKey for DBType {
    type Value = Arc<RwLock<Database>>;
}

pub struct RedisManager;

impl TypeMapKey for RedisManager {
    type Value = Arc<Mutex<Redis>>;
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

#[tokio::main]
async fn main() {
    kankyo::load(false).expect("Failed to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let owners = match http.get_current_application_info().await {
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

    let redis = Arc::new(tokio::sync::Mutex::new(
       Redis::open().expect("Couldn't open redis")
    ));
    // redis.lock().await.test().await.expect("redis test failed!");

    let framework = 
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP)
            // .group(&VOICE_GROUP)
            .group(&ADMIN_GROUP)
            .help(&CDPR_HELP);

    let mut client = Client::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // let buffer_map = Arc::new(RwLock::new(DiscordAudioBuffer::new(BUFFER_LENGTH)));

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write().await;
        data.insert::<DBType>(db.clone());
        data.insert::<RedisManager>(redis.clone());
        data.insert::<VoiceManager>(client.voice_manager.clone());
        // data.insert::<BufferType>(Arc::clone(&buffer_map));
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(err) = client.start().await {
        println!("An Error ocurred while running the client: {:?}", err);
    }
}
