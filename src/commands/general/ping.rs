use crate::commands::command_prelude::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let shard_manager = match data.get::<crate::data::ShardManagerContainer>() {
        Some(v) => Ok(v),
        None => {
            msg.channel_id
                .say(&ctx, "There was a problem getting the shard manager")?;

            Err(CommandError("Failed to get shard manager".to_string()))
        }
    }?;

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => Ok(runner),
        None => {
            msg.channel_id.say(&ctx, "No shard found")?;

            Err(CommandError("Failed to get current shard".to_string()))
        }
    }?;

    if let Some(latency) = runner.latency {
        msg.channel_id.say(
            &ctx,
            &format!(
                "{}'s ping is {}ms",
                ctx.cache.read().user.name,
                latency.as_millis(),
            ),
        )?;
    } else {
        warn!("User attempted to get latency, but it was not available");
        msg.channel_id
            .say(&ctx, "Unable to get latency, try again later")?;
    }

    Ok(())
}
