command!(about(_ctx, msg, _args) {
    if let Err(why) = msg.channel_id.say("This is Mr. CD Projekt Red. He will do things in the near future.") {
        error!("Error sending message: {:?}", why);
    }
});
