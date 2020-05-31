use crate::common::store::ShardManagerContainer;
use serenity::{
    client::bridge::gateway::ShardId,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context,
};

#[command]
fn ping(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = ctx.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

            return Ok(());
        }
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = msg.reply(&ctx, "No shard found");

            return Ok(());
        }
    };

    let _ = msg.reply(
        &ctx,
        &format!(
            "The shard latency is {}ms",
            runner.latency.unwrap_or_default().as_millis()
        ),
    );

    Ok(())
}
