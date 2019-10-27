use crate::inserts::{ShardManagerContainer, UptimeKey};
use log::error;
use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

group!({
    name: "bot",
    options: {},
    commands: [uptime, ping]
});

#[command]
#[description("Get the bots uptime")]
#[usage("")]
#[bucket = "uptime"]
fn uptime(ctx: &mut Context, msg: &Message) -> CommandResult {
    use chrono::Duration;

    let data = ctx.data.read();

    if let Some(uptime) = data.get::<UptimeKey>() {
        let (days, hours, minutes, seconds) = {
            // I couldn't find another way to do it, so I get the days/hours and subtract them from `uptime` before returning the days/hours
            let mut uptime = uptime.get();

            let days = {
                let days = uptime.num_days();
                uptime = uptime
                    .checked_sub(&Duration::days(days))
                    .unwrap_or_else(|| Duration::days(0));
                days
            };

            let hours = {
                let hours = uptime.num_hours();
                uptime = uptime
                    .checked_sub(&Duration::hours(hours))
                    .unwrap_or_else(|| Duration::hours(0));
                hours
            };

            let minutes = {
                let minutes = uptime.num_minutes();
                uptime = uptime
                    .checked_sub(&Duration::minutes(minutes))
                    .unwrap_or_else(|| Duration::minutes(0));
                minutes
            };

            let seconds = uptime.num_seconds();

            (days, hours, minutes, seconds)
        };

        if let Err(err) = msg.channel_id.say(
            &ctx.http,
            &format!(
                "{bot_name} has been online for {} days, {} hours, {} minutes and {} seconds",
                days,
                hours,
                minutes,
                seconds,
                bot_name = if let Ok(app) = ctx.http.get_current_application_info() {
                    app.name
                } else {
                    "This bot".to_owned()
                },
            ),
        ) {
            error!("Message Error: {:?}", err);
        }
    } else if let Err(err) = msg
        .channel_id
        .say(&ctx.http, "Failed to get my uptime, please try again later")
    {
        error!("Message Error: {:?}", err);
    }

    Ok(())
}

#[command]
#[description("Get the bots ping")]
#[usage("")]
#[bucket = "ping"]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    let shard_manager = if let Some(container) = data.get::<ShardManagerContainer>() {
        container
    } else {
        if let Err(err) = msg.reply(&ctx, "I couldn't get my ping, try again later!") {
            error!("Message Error: {:?}", err);
        }
        return Ok(());
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    let shard = if let Some(shard) = runners.get(&ShardId(ctx.shard_id)) {
        shard
    } else {
        if let Err(err) = msg.reply(&ctx, "I couldn't get my ping, try again later!") {
            error!("Message Error: {:?}", err);
        }
        return Ok(());
    };

    match shard.latency {
        Some(latency) => {
            if let Err(err) = msg.reply(&ctx, &format!("Pong! {:?}", latency)) {
                error!("Message Error: {:?}", err);
            }
        }

        None => {
            if let Err(err) = msg.reply(&ctx, "I couldn't get my ping, try again later!") {
                error!("Message Error: {:?}", err);
            }
        }
    }

    Ok(())
}
