use crate::commands::command_prelude::*;

#[command]
#[description = "Restart the bot"]
#[aliases("shut_down")]
pub fn shutdown(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "Shutting down...")?;
    msg.react(&ctx, "✅")?;

    info!(
        "{} ({}) requested a shutdown",
        msg.author.tag(),
        msg.author.id
    );
    info!("Shutting down...");

    let data = ctx.data.read();

    let shard_manager =
        if let Some(shard_manager) = data.get::<crate::data::ShardManagerContainer>() {
            Ok(shard_manager)
        } else {
            Err(CommandError(
                "Failed to get shard manager while shutting down".to_string(),
            ))
        }?;

    shard_manager.lock().shutdown_all();

    Ok(())
}
