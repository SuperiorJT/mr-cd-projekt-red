use crate::model::user::User;
use crate::DBType;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::Message;
use serenity::prelude::*;

#[command]
#[description = "Registers the user to the database"]
#[usage("~register")]
pub fn register(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.reply(
        ctx,
        &format!("Attempting to register user {}", msg.author.name),
    );

    let db_lock = ctx
        .data
        .read()
        .get::<DBType>()
        .cloned()
        .expect("Expected Database in ShareMap");
    let db = db_lock
        .read()
        .expect("Expected Database lock to not be poisoned");

    let user = User::new(msg.author.id);

    user.db_create(&db).expect("Failed to create user");

    Ok(())
}