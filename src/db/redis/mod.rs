use std::{collections::HashMap, hash::Hash};

use redis::AsyncCommands;
use serenity::model::prelude::*;
mod structs;
pub struct Redis {
    pub client: redis::Client,
}

impl Redis {
    pub fn open() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;

        Ok(Redis { client })
    }

    pub async fn get_connection(&self) -> redis::RedisResult<redis::aio::Connection> {
        self.client.get_async_connection().await
    }

    pub async fn guild_update(&self, guild: &Guild) -> redis::RedisResult<()> {
        let guild_key = format!("guild:{}", guild.id);
        let mut con = self.get_connection().await?;

        redis::cmd("JSON.SET")
            .arg(guild_key)
            .arg(".")
            .arg(
                serde_json::to_string(&structs::RedisGuild::from(guild))
                    .expect("Failed to serialize guild"),
            )
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn channel_set(&self, channel: &GuildChannel) -> redis::RedisResult<()> {
        let guild_key = format!("guild:{}", channel.guild_id);
        let mut con = self.get_connection().await?;

        redis::cmd("JSON.SET")
            .arg(guild_key)
            .arg(format!(".channels[\"{}\"]", channel.id))
            .arg(serde_json::to_string(channel).expect("Failed to serialize channel"))
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn channel_delete(&self, channel: &GuildChannel) -> redis::RedisResult<()> {
        let guild_key = format!("guild:{}", channel.guild_id);
        let mut con = self.get_connection().await?;

        redis::cmd("JSON.DEL")
            .arg(guild_key)
            .arg(format!(".channels[\"{}\"]", channel.id))
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn emojis_update(
        &self,
        guild_id: GuildId,
        emojis: HashMap<EmojiId, Emoji>,
    ) -> redis::RedisResult<()> {
        let guild_key = format!("guild:{}", guild_id);
        let mut con = self.get_connection().await?;

        let before_json: String = redis::cmd("JSON.GET")
            .arg(&guild_key)
            .arg(".emojis")
            .query_async(&mut con)
            .await?;

        let before: HashMap<EmojiId, Emoji> = serde_json::from_str(&before_json).unwrap();

        let comparable_before: HashMap<EmojiId, MyEmoji> = before
            .clone()
            .into_iter()
            .map(|(k, v)| (k, MyEmoji(v)))
            .collect();

        let comparable_after: HashMap<EmojiId, MyEmoji> = emojis
            .clone()
            .into_iter()
            .map(|(k, v)| (k, MyEmoji(v)))
            .collect();

        let diff = comparable_before.diff(&comparable_after);

        // TODO: PubSub - Emit the diff

        redis::cmd("JSON.SET")
            .arg(&guild_key)
            .arg(".emojis")
            .arg(serde_json::to_string(&emojis).expect("Failed to serialize emojis"))
            .query_async(&mut con)
            .await?;

        Ok(())
    }

    pub async fn test(&self) -> redis::RedisResult<()> {
        let mut con = self.get_connection().await?;

        con.set("test", 12).await?;
        Ok(())
    }
}

#[derive(Debug)]
struct SimpleChange<T> {
    pub previous_value: T,
    pub current_value: T,
}

#[derive(Debug)]
struct CollectionDiffChange<K, V> {
    pub added: HashMap<K, V>,
    pub removed: HashMap<K, V>,
    pub changed: HashMap<K, SimpleChange<V>>,
}
trait CollectionDiff<T> {
    type Key;
    type Value;

    fn diff(&self, after: &T) -> CollectionDiffChange<Self::Key, Self::Value>;
}

impl<K, V> CollectionDiff<HashMap<K, V>> for HashMap<K, V>
where
    K: Eq + Hash + Copy,
    V: Eq + PartialEq + Clone,
{
    type Key = K;
    type Value = V;
    /// Returns a CollectionDiffChange containing added and removed entries
    fn diff(&self, after: &HashMap<K, V>) -> CollectionDiffChange<K, V> {
        let mut added = HashMap::new();
        let mut removed = HashMap::new();
        let mut changed = HashMap::new();

        self.keys().for_each(|k| {
            if !after.contains_key(k) {
                removed.insert(*k, self.get(k).unwrap().clone());
            } else {
                let prev = self.get(k).unwrap();
                let new = after.get(k).unwrap();
                if prev != new {
                    changed.insert(
                        k.clone(),
                        SimpleChange {
                            previous_value: prev.clone(),
                            current_value: new.clone(),
                        },
                    );
                }
            }
        });

        after.keys().for_each(|k| {
            if !self.contains_key(k) {
                added.insert(*k, after.get(k).unwrap().clone());
            }
        });

        CollectionDiffChange {
            added,
            removed,
            changed,
        }
    }
}

#[derive(Clone, Debug)]
struct MyEmoji(Emoji);

impl Eq for MyEmoji {}

impl PartialEq for MyEmoji {
    fn eq(&self, other: &Self) -> bool {
        if self.0.animated != other.0.animated {
            return false;
        }
        if self.0.id != other.0.id {
            return false;
        }
        if self.0.name != other.0.name {
            return false;
        }
        if self.0.managed != other.0.managed {
            return false;
        }
        if self.0.require_colons != other.0.require_colons {
            return false;
        }
        if self.0.roles != other.0.roles {
            return false;
        }
        true
    }
}
