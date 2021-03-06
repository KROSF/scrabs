use crate::common::{store::VoiceManager, utils::check_msg};

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
fn undeafen(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.read().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "Error finding channel info"));

            return Ok(());
        }
    };

    let manager_lock = ctx
        .data
        .read()
        .get::<VoiceManager>()
        .cloned()
        .expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        handler.deafen(false);

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened"));
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to undeafen in"),
        );
    }

    Ok(())
}
