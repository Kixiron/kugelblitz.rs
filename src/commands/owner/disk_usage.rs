use crate::commands::command_prelude::*;

#[command]
#[description = "See the disk space used by the bots' database"]
pub fn usage(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let db = if let Some(db) = data.get::<crate::data::DatabaseContainer>() {
        Ok(db)
    } else {
        Err(CommandError("Failed to get database".to_string()))
    }?;

    // Usage in bytes
    let bytes = match db.read().size_on_disk() {
        Ok(bytes) => Ok(bytes),
        Err(err) => Err(CommandError(format!("Failed to get disk usage: {:?}", err))),
    }?;

    let usage = if bytes < 1000 {
        if bytes == 1 {
            format!("{} byte", bytes)
        } else {
            format!("{} bytes", bytes)
        }
    } else if bytes < 1000_u64.pow(2) {
        let bytes = bytes / 1000;
        if bytes == 1 {
            format!("{} kilobyte", bytes)
        } else {
            format!("{} kilobytes", bytes)
        }
    } else if bytes < 1000_u64.pow(3) {
        let bytes = bytes / 1000 / 1000;
        if bytes == 1 {
            format!("{} megabyte", bytes)
        } else {
            format!("{} megabytes", bytes)
        }
    } else if bytes < 1000_u64.pow(4) {
        let bytes = bytes / 1000 / 1000 / 1000;
        if bytes == 1 {
            format!("{} gigabyte", bytes)
        } else {
            format!("{} gigabytes", bytes)
        }
    } else {
        format!("{} terabytes", bytes / 1000 / 1000 / 1000 / 1000)
    };

    info!("Current database disk usage: {}", usage);

    msg.channel_id
        .say(&ctx, &format!("Current database disk usage: {}", usage))?;

    Ok(())
}
