use std::collections::HashMap;

use serenity::{
    async_trait, client::bridge::gateway::event::ShardStageUpdateEvent, client::EventHandler,
    model::prelude::*, prelude::*,
};

use crate::RedisManager;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        let data = ctx.data.read().await;
        let redis_lock = data
            .get::<RedisManager>()
            .expect("Expected Redis in ShareMap")
            .clone();

        let r = redis_lock.lock().await;
        for guild_id in guilds {
            let guild = match guild_id.to_guild_cached(&ctx.cache).await {
                Some(g) => g,
                None => {
                    continue;
                }
            };
            r.guild_update(&guild)
                .await
                .expect(&format!("Failed to cache guild: {}", guild_id));
        }
    }

    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        let data = ctx.data.read().await;
        let redis_lock = data
            .get::<RedisManager>()
            .expect("Expected Redis in ShareMap")
            .clone();

        let r = redis_lock.lock().await;
        r.channel_set(channel)
            .await
            .expect(&format!("Failed to cache channel: {}", channel.id));
    }

    async fn category_create(&self, _ctx: Context, _category: &ChannelCategory) {}

    async fn category_delete(&self, _ctx: Context, _category: &ChannelCategory) {}

    async fn private_channel_create(&self, _ctx: Context, _channel: &PrivateChannel) {}

    async fn channel_delete(&self, ctx: Context, channel: &GuildChannel) {
        let data = ctx.data.read().await;
        let redis_lock = data
            .get::<RedisManager>()
            .expect("Expected Redis in ShareMap")
            .clone();

        let r = redis_lock.lock().await;
        r.channel_delete(channel)
            .await
            .expect(&format!("Failed to cache delete channel: {}", channel.id));
    }

    async fn channel_pins_update(&self, _ctx: Context, _pin: ChannelPinsUpdateEvent) {}

    async fn channel_recipient_addition(&self, _ctx: Context, _group_id: ChannelId, _user: User) {}

    async fn channel_recipient_removal(&self, _ctx: Context, _group_id: ChannelId, _user: User) {}

    async fn channel_update(&self, ctx: Context, _old: Option<Channel>, new: Channel) {
        match new {
            Channel::Guild(channel) => {
                self.channel_create(ctx, &channel).await;
            }
            _ => {}
        }
    }

    async fn guild_ban_addition(&self, _ctx: Context, _guild_id: GuildId, _banned_user: User) {}

    async fn guild_ban_removal(&self, _ctx: Context, _guild_id: GuildId, _unbanned_user: User) {}

    async fn guild_create(&self, _ctx: Context, _guild: Guild, _is_new: bool) {}

    async fn guild_delete(&self, _ctx: Context, _incomplete: PartialGuild, _full: Option<Guild>) {}

    async fn guild_emojis_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _current_state: HashMap<EmojiId, Emoji>,
    ) {
    }

    async fn guild_integrations_update(&self, _ctx: Context, _guild_id: GuildId) {}

    async fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, _new_member: Member) {}

    async fn guild_member_removal(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _user: User,
        _member_data_if_available: Option<Member>,
    ) {
    }

    async fn guild_member_update(
        &self,
        _ctx: Context,
        _old_if_available: Option<Member>,
        _new: Member,
    ) {
    }

    async fn guild_members_chunk(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _offline_members: std::collections::HashMap<UserId, Member>,
        _nonce: Option<String>,
    ) {
    }

    async fn guild_role_create(&self, _ctx: Context, _guild_id: GuildId, _new: Role) {}

    async fn guild_role_delete(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _removed_role_id: RoleId,
        _removed_role_data_if_available: Option<Role>,
    ) {
    }

    async fn guild_role_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _old_data_if_available: Option<Role>,
        _new: Role,
    ) {
    }

    async fn guild_unavailable(&self, _ctx: Context, _guild_id: GuildId) {}

    async fn guild_update(
        &self,
        _ctx: Context,
        _old_data_if_available: Option<Guild>,
        _new_but_incomplete: PartialGuild,
    ) {
    }

    async fn invite_create(&self, _ctx: Context, _data: InviteCreateEvent) {}

    async fn invite_delete(&self, _ctx: Context, _data: InviteDeleteEvent) {}

    async fn message(&self, _ctx: Context, _new_message: Message) {}

    async fn message_delete(
        &self,
        _ctx: Context,
        _channel_id: ChannelId,
        _deleted_message_id: MessageId,
    ) {
    }

    async fn message_delete_bulk(
        &self,
        _ctx: Context,
        _channel_id: ChannelId,
        _multiple_deleted_messages_ids: Vec<MessageId>,
    ) {
    }

    async fn message_update(
        &self,
        _ctx: Context,
        _old_if_available: Option<Message>,
        _new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
    }

    async fn reaction_add(&self, _ctx: Context, _add_reaction: Reaction) {}

    async fn reaction_remove(&self, _ctx: Context, _removed_reaction: Reaction) {}

    async fn reaction_remove_all(
        &self,
        _ctx: Context,
        _channel_id: ChannelId,
        _removed_from_message_id: MessageId,
    ) {
    }

    async fn presence_replace(&self, _ctx: Context, _: Vec<Presence>) {}

    async fn presence_update(&self, _ctx: Context, _new_data: PresenceUpdateEvent) {}

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {}

    async fn shard_stage_update(&self, _ctx: Context, _: ShardStageUpdateEvent) {}

    async fn typing_start(&self, _ctx: Context, _: TypingStartEvent) {}

    async fn user_update(&self, _ctx: Context, _old_data: CurrentUser, _new: CurrentUser) {}

    async fn voice_server_update(&self, _ctx: Context, _: VoiceServerUpdateEvent) {}

    async fn voice_state_update(
        &self,
        _ctx: Context,
        _: Option<GuildId>,
        _old: Option<VoiceState>,
        _new: VoiceState,
    ) {
    }

    async fn webhook_update(
        &self,
        _ctx: Context,
        _guild_id: GuildId,
        _belongs_to_channel_id: ChannelId,
    ) {
    }
}
