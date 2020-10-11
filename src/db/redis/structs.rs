use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Serialize;
use serenity::model::prelude::*;

#[derive(Clone, Debug, Serialize)]
pub struct RedisGuild {
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: u64,
    pub application_id: Option<ApplicationId>,
    pub channels: HashMap<ChannelId, GuildChannel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub emojis: HashMap<EmojiId, Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    pub id: GuildId,
    pub joined_at: DateTime<Utc>,
    pub large: bool,
    pub member_count: u64,
    pub members: HashMap<UserId, Member>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub owner_id: UserId,
    pub presences: HashMap<UserId, Presence>,
    pub region: String,
    pub roles: HashMap<RoleId, Role>,
    pub splash: Option<String>,
    pub system_channel_id: Option<ChannelId>,
    pub verification_level: VerificationLevel,
    pub voice_states: HashMap<UserId, VoiceState>,
    pub description: Option<String>,
    #[serde(default)]
    pub premium_tier: PremiumTier,
    #[serde(default)]
    pub premium_subscription_count: u64,
    pub banner: Option<String>,
    pub vanity_url_code: Option<String>,
    pub preferred_locale: String,
}

impl From<&Guild> for RedisGuild {
    fn from(g: &Guild) -> Self {
        let g = g.clone();
        RedisGuild {
            afk_channel_id: g.afk_channel_id,
            afk_timeout: g.afk_timeout,
            application_id: g.application_id,
            channels: g.channels,
            default_message_notifications: g.default_message_notifications,
            emojis: g.emojis,
            explicit_content_filter: g.explicit_content_filter,
            features: g.features,
            icon: g.icon,
            id: g.id,
            joined_at: g.joined_at,
            large: g.large,
            member_count: g.member_count,
            members: g.members,
            mfa_level: g.mfa_level,
            name: g.name,
            owner_id: g.owner_id,
            presences: g.presences,
            region: g.region,
            roles: g.roles,
            splash: g.splash,
            system_channel_id: g.system_channel_id,
            verification_level: g.verification_level,
            voice_states: g.voice_states,
            description: g.description,
            premium_tier: g.premium_tier,
            premium_subscription_count: g.premium_subscription_count,
            banner: g.banner,
            vanity_url_code: g.vanity_url_code,
            preferred_locale: g.preferred_locale,
        }
    }
}
