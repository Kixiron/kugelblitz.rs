use crate::commands::command_prelude::*;

#[command]
#[description = "Restart the bot"]
pub fn restart(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "Restarting...")?;
    msg.react(&ctx, "✅")?;

    info!("Restarting shard No. {}...", ctx.shard_id + 1);

    let data = ctx.data.read();

    let shard_manager =
        if let Some(shard_manager) = data.get::<crate::data::ShardManagerContainer>() {
            Ok(shard_manager)
        } else {
            Err(CommandError(
                "Failed to get shard manager while restarting shard".to_string(),
            ))
        }?;

    shard_manager.lock().restart(ShardId(ctx.shard_id));

    Ok(())
}
