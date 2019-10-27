use crate::inserts::ShardManagerContainer;
use log::{error, info};
use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
};
use std::string::ToString;

group!({
    name: "owner",
    options: {
        owners_only: true,
        prefixes: ["root"],
    },
    commands: [quit, status, restart, restart_current]
});

#[command]
#[owners_only]
#[aliases("exit", "shutdown", "killall")]
#[description("Make the bot shut down all processes")]
#[usage("")]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    save_all!(data);

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        if let Err(err) = msg.reply(&ctx, "Shutting down...") {
            error!("Reply error: {:?}", err);
        }

        manager.lock().shutdown_all();
    } else if let Err(err) = msg.reply(&ctx, "There was a problem getting the shard manager") {
        error!("Reply error: {:?}", err);
    }

    Ok(())
}

#[command]
#[owners_only]
#[description("Set the status of the bot")]
#[usage("[online, idle, dnd, invisible, offline]")]
fn status(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    use serenity::model::user::OnlineStatus;

    let status = {
        if let Ok(status) = args.single::<String>() {
            match status.trim().to_lowercase().as_str() {
                "online" => OnlineStatus::Online,
                "idle" => OnlineStatus::Idle,
                "dnd" => OnlineStatus::DoNotDisturb,
                "invisible" => OnlineStatus::Invisible,
                "offline" => OnlineStatus::Offline,
                _ => {
                    if let Err(err) = msg.channel_id.say(
                        &ctx.http,
                        "That's not a valid status, try `online`, `idle`, `dnd`, `invisible` or `offline`",
                    ) {
                        error!("Reply error: {:?}", err);
                    }
                    return Ok(());
                }
            }
        } else {
            if let Err(err) = msg.channel_id.say(
                &ctx.http,
                "You need to supply a valid status, try `online`, `idle`, `dnd`, `invisible` or `offline`",
            ) {
                error!("Reply error: {:?}", err);
            }
            return Ok(());
        }
    };

    ctx.set_presence(None, status);
    if let Err(err) = msg
        .channel_id
        .say(&ctx.http, format!("Set status to `{:?}`", status))
    {
        error!("{:?}", err);
    }
    info!("Set status to {:?}", status);

    Ok(())
}

#[command]
#[owners_only]
fn restart(ctx: &mut Context, msg: &Message) -> CommandResult {
    use std::env;

    let data = ctx.data.read();

    save_all!(data);

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        info!("Restarting all shards");

        env::set_var(
            "DISCORD_BOT_RESTART_MESSAGE",
            match msg.channel_id.say(&ctx, "Restarting...") {
                Ok(msg) => [*msg.id.as_u64(), *msg.channel_id.as_u64()]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
                Err(err) => {
                    error!("Message Error: {:?}", err);
                    "".to_string()
                }
            },
        );

        let mut manager = manager.lock();
        for shard in &manager.shards_instantiated() {
            info!("Restarting Shard #{}", shard.0);
            manager.restart(*shard);
        }
    } else if let Err(err) = msg.reply(&ctx, "There was a problem getting the shard manager") {
        error!("Message Error: {:?}", err);
    }

    Ok(())
}

#[command]
#[owners_only]
fn restart_current(ctx: &mut Context, msg: &Message) -> CommandResult {
    use std::env;

    let data = ctx.data.read();

    save_all!(data);

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        info!("Restarting shard #{}", ctx.shard_id);

        env::set_var(
            "DISCORD_BOT_RESTART_MESSAGE",
            match msg.channel_id.say(&ctx, "Restarting...") {
                Ok(msg) => [*msg.id.as_u64(), *msg.channel_id.as_u64()]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
                Err(err) => {
                    error!("Message Error: {:?}", err);
                    "".to_string()
                }
            },
        );

        manager.lock().restart(ShardId(ctx.shard_id));
    } else if let Err(err) = msg.reply(&ctx, "There was a problem getting the shard manager") {
        error!("Message Error: {:?}", err);
    }

    Ok(())
}

// TODO: Port commands from Kugelblitz
