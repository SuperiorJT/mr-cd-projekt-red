use serenity::{
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::Context,
};
use std::collections::HashSet;

#[help]
pub async fn cdpr_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
