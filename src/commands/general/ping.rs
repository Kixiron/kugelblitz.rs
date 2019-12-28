use log::{error, warn};
use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let shard_manager = match data.get::<crate::data::ShardManagerContainer>() {
        Some(v) => v,
        None => {
            error!("An attempt to lock the shard manager was made, but it failed");
            if let Err(err) = msg.reply(&ctx, "There was a problem getting the shard manager") {
                error!("Error sending message: {:?}", err);
            }

            return Ok(());
        }
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            error!("An attempt to get the current shard was made, but it failed");
            if let Err(err) = msg.reply(&ctx, "No shard found") {
                error!("Error sending message: {:?}", err);
            }

            return Ok(());
        }
    };

    if let Some(latency) = runner.latency {
        if let Err(err) = msg.reply(
            &ctx,
            &format!(
                "{}'s ping is {}ms",
                ctx.cache.read().user.name,
                latency.as_millis(),
            ),
        ) {
            error!("Error sending message: {:?}", err);
        }
    } else {
        warn!("User attempted to get latency, but it was not available");
        if let Err(err) = msg.reply(&ctx, "Unable to get latency, try again later") {
            error!("Error sending message: {:?}", err);
        }
    }

    Ok(())
}
