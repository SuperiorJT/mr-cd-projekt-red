use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::guild::Guild;
use serenity::model::prelude::{ChannelId, Member, Message, RoleId, User};
use serenity::prelude::*;

pub static SHADOW_REALM_CHANNEL_ID: ChannelId = ChannelId(215294075090370560);

pub static BANISHED_ROLE_ID: RoleId = RoleId(583472301199327242);

enum CommandError {
    GuildNotFound,
    UserNotInVoice,
    AuthorNotInVoice,
}

#[command]
#[description = "Sends another user to the shadow realm for punishment."]
#[usage("@Victim @AnotherVictim ...")]
#[aliases("sr", "shadow")]
#[min_args(1)]
#[owners_only]
pub fn shadow_realm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.len() == 0 || msg.mentions.len() == 0 {
        send_msg(
            ctx,
            msg,
            format!("Please mention the user(s) to be punished."),
        );
        return Ok(());
    }
    let usernames = format_usernames(msg);

    // Status update
    send_msg(
        ctx,
        msg,
        format!("Attempting to send {} to the shadow realm...", usernames),
    );

    match get_voice_states(ctx, msg, &msg.mentions) {
        Ok(mut user_states) => {
            let guild_lock = msg
                .guild(&ctx.cache)
                .expect("Guild not found. This should never happen.");
            let guild = guild_lock.read();
            let mut members = user_states
                .drain(..)
                .map(|(user, channel_id)| {
                    if let Ok(member) = guild.member(&ctx, user.id) {
                        return Some((user, member, channel_id));
                    }
                    None
                })
                .filter(|option| option.is_some())
                .map(|option| option.unwrap())
                .collect::<Vec<(User, Member, ChannelId)>>();
            members.iter_mut().for_each(|(user, member, _channel_id)| {
                if user.id == msg.author.id {
                    move_user_to_channel(ctx, &guild, &user, SHADOW_REALM_CHANNEL_ID)
                        .expect(&format!("Couldn't move user! {}", user.name));
                } else {
                    assign_banishment(ctx, &guild, &user, member)
                        .expect(&format!("Couldn't banish user! {}", user.name));
                }
            });
            let mut punish_msg = format!("{} are being punished for 5 seconds...", usernames);
            if msg.mentions.len() == 1 {
                punish_msg = format!("{} is being punished for 5 seconds...", usernames);
            }

            send_msg(ctx, msg, punish_msg);

            thread::sleep(Duration::from_millis(5000));

            members.iter_mut().for_each(|(user, member, channel_id)| {
                if user.id == msg.author.id {
                    move_user_to_channel(ctx, &guild, &user, *channel_id)
                        .expect(&format!("Couldn't move user back! {}", user.name));
                } else {
                    unassign_banishment(ctx, &guild, &user, member, *channel_id)
                        .expect(&format!("Couldn't unbanish user! {}", user.name));
                }
            });
        }
        Err(CommandError::GuildNotFound) => {
            send_msg(
                ctx,
                msg,
                format!("I can't find your server! Try again later."),
            );
        }
        Err(CommandError::AuthorNotInVoice) => {
            send_msg(
                ctx,
                msg,
                format!("You need to be in voice to punish others!"),
            );
        }
        Err(CommandError::UserNotInVoice) => {
            send_msg(ctx, msg, format!("Mentioned users are not in voice."));
        }
    }
    Ok(())
}

fn send_msg(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    if let Err(why) = msg.channel_id.say(ctx, content) {
        error!("Error sending message: {:?}", why);
    }
}

fn format_usernames(msg: &Message) -> String {
    msg.mentions
        .iter()
        .rev()
        .fold((String::new(), 0), |acc, user| {
            if msg.mentions.len() > 1 && msg.mentions.len() - acc.1 == 1 {
                return (acc.0 + "and " + &user.name, acc.1 + 1);
            }
            if msg.mentions.len() > 2 {
                return (acc.0 + &user.name + ", ", acc.1 + 1);
            }
            if msg.mentions.len() == 1 {
                return (acc.0 + &user.name, acc.1 + 1);
            }
            (acc.0 + &user.name + " ", acc.1 + 1)
        })
        .0
}

fn get_voice_states(
    ctx: &Context,
    msg: &Message,
    users: &Vec<User>,
) -> Result<Vec<(User, ChannelId)>, CommandError> {
    match msg.guild(&ctx.cache) {
        Some(guild) => {
            if let Ok(id) = get_user_voice_channel(&guild, &msg.author) {
                let mut valid_users = users.iter().fold(vec![], |mut acc, user| {
                    if user.id == msg.author.id {
                        return acc;
                    }
                    if let Ok(id) = get_user_voice_channel(&guild, &user) {
                        acc.push((user.clone(), id));
                    }
                    acc
                });
                if valid_users.len() == 0 {
                    return Err(CommandError::UserNotInVoice);
                }
                valid_users.push((msg.author.clone(), id));
                Ok(valid_users)
            } else {
                return Err(CommandError::AuthorNotInVoice);
            }
        }
        None => Err(CommandError::GuildNotFound),
    }
}

fn get_user_voice_channel(
    guild: &Arc<RwLock<Guild>>,
    user: &User,
) -> Result<ChannelId, CommandError> {
    if let Some(state) = guild.read().voice_states.get(&user.id) {
        if let Some(id) = state.channel_id {
            Ok(id)
        } else {
            Err(CommandError::UserNotInVoice)
        }
    } else {
        Err(CommandError::UserNotInVoice)
    }
}

fn move_user_to_channel(
    ctx: &Context,
    guild: &Guild,
    user: &User,
    channel_id: ChannelId,
) -> serenity::Result<()> {
    guild.move_member(&ctx.http, user.id, channel_id)
}

fn assign_banishment(
    ctx: &Context,
    guild: &Guild,
    user: &User,
    member: &mut Member,
) -> serenity::Result<()> {
    guild.edit_member(&ctx.http, user.id, |edit_member| {
        member.roles.push(BANISHED_ROLE_ID);
        edit_member.voice_channel(SHADOW_REALM_CHANNEL_ID);
        edit_member.roles(&member.roles)
    })
}

fn unassign_banishment(
    ctx: &Context,
    guild: &Guild,
    user: &User,
    member: &mut Member,
    channel_id: ChannelId,
) -> serenity::Result<()> {
    member.roles = member
        .roles
        .drain(..)
        .filter(|role_id| *role_id != BANISHED_ROLE_ID)
        .collect::<Vec<RoleId>>();
    let res = guild.edit_member(&ctx.http, user.id, |edit_member| {
        edit_member.voice_channel(channel_id);
        edit_member.roles(&member.roles)
    });
    if res.is_ok() {
        return res;
    }
    guild.edit_member(&ctx.http, user.id, |edit_member| {
        edit_member.roles(&member.roles)
    })
}
