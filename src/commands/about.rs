use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command
};

#[command]
#[description = "Provides a description about the bot"]
#[usage("~about")]
pub fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(ctx, "This is Mr. CD Projekt Red. He will do things in the near future.") {
        error!("Error sending message: {:?}", why);
    }
    Ok(())
}
