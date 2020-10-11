use redis::AsyncCommands;
use serenity::model::{channel::GuildChannel, guild::Guild};
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

    pub async fn channel_create(&self, channel: &GuildChannel) -> redis::RedisResult<()> {
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

    pub async fn test(&self) -> redis::RedisResult<()> {
        let mut con = self.get_connection().await?;

        con.set("test", 12).await?;
        Ok(())
    }
}
