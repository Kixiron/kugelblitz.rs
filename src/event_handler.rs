use crate::inserts::{DispatchEvent, DispatcherKey};
use log::{error, info, warn};
use serenity::{
    client::{Context, EventHandler},
    framework::standard::{CommandError, DispatchError, Reason},
    model::{
        channel::Message,
        event::MessageUpdateEvent,
        gateway::Ready,
        id::{ChannelId, GuildId, MessageId},
        permissions::Permissions,
    },
};
use std::env;

pub struct Handler;

// TODO: Add more Events
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _ready: Ready) {
        match env::var("DISCORD_BOT_RESTART_MESSAGE") {
            Ok(ref val) if !val.is_empty() => {
                info!(
                    "Shard #{} Restarted on {}",
                    ctx.shard_id,
                    chrono::offset::Local::now().format("%a %b %0e %T %Y")
                );

                let (message_id, channel_id) = {
                    let ids = val
                        .split(',')
                        .map(|id| id.parse::<u64>().unwrap_or(0))
                        .collect::<Vec<u64>>();
                    (MessageId(ids[0]), ChannelId(ids[1]))
                };

                if message_id != 0 && channel_id != 0 {
                    if let Ok(mut message) = channel_id.message(&ctx.http, message_id) {
                        if let Err(err) = message
                            .edit(&ctx, |message| message.content("Restarting... Restarted!"))
                        {
                            error!("Edit Error: {:?}", err);
                        }
                    }
                } else {
                    error!(
                        "Either the Restart Message or Channel id failed to parse: `{:?}`",
                        val
                    );
                }

                env::set_var("DISCORD_BOT_RESTART_MESSAGE", "");
            }
            _ => info!(
                "Shard #{} Connected on {}",
                ctx.shard_id,
                chrono::offset::Local::now().format("%a %b %0e %T %Y")
            ),
        }
    }

    fn message(&self, ctx: Context, msg: Message) {
        // Special case for special server
        if msg.guild_id == Some(GuildId(511_254_259_753_418_763))
            && msg.content.to_lowercase().contains("sean")
        {
            if let Err(err) = msg.reply(&ctx, "*Saevyn") {
                error!("Message Error: {:?}", err);
            }
        }

        let mut ctx = ctx.data.write();

        let dispatcher = if let Some(dispatcher) = ctx.get_mut::<DispatcherKey>() {
            dispatcher
        } else {
            error!("Failed to get Dispatcher");
            return;
        };

        let mut dispatcher = dispatcher.write();

        // Dispatch the Message event
        match &mut dispatcher {
            Ok(dispatcher) => dispatcher.dispatch_event(&DispatchEvent::Message(msg.id)),
            Err(err) => error!("Dispatcher Error: {:?}", err),
        }
    }

    fn message_update(
        &self,
        ctx: Context,
        _old_if_available: Option<Message>,
        new: Option<Message>,
        _event: MessageUpdateEvent,
    ) {
        if let Some(new) = new {
            if new.guild_id == Some(GuildId(511_254_259_753_418_763))
                && new.content.to_lowercase().contains("sean")
            {
                if let Err(err) = new.reply(&ctx, "*Saevyn") {
                    error!("Message Error: {:?}", err);
                }
            }
        }
    }
}

pub fn configure_dispatch_error(ctx: &mut Context, msg: &Message, error: DispatchError) {
    use serenity::framework::standard::DispatchError::*;

    // TODO: Add more variants
    match error {
        Ratelimited(seconds) => ratelimited(ctx, msg, seconds),
        // TODO: Find out what message is
        CommandDisabled(message) => command_disabled(ctx, msg, &message),
        BlockedUser => blocked_user(ctx, msg),
        BlockedGuild => blocked_guild(ctx, msg),
        OnlyForDM => only_for_dm(ctx, msg),
        OnlyForGuilds => only_for_guilds(ctx, msg),
        LackingRole => lacking_role(ctx, msg),
        NotEnoughArguments { min, given } => not_enough_arguments(ctx, msg, min, given),
        TooManyArguments { max, given } => too_many_arguments(ctx, msg, max, given),
        CheckFailed(message, reason) => check_failed(ctx, msg, message, &reason),
        LackingPermissions(permissions) => lacking_permissions(ctx, msg, permissions),
        OnlyForOwners | WebhookAuthor | IgnoredBot | BlockedChannel | _ => {}
    }
}

#[inline]
fn lacking_role(ctx: &mut Context, msg: &Message) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        "I'm sorry, but you don't have the correct role to use that command",
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn only_for_guilds(ctx: &mut Context, msg: &Message) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        "I'm sorry, but that command is only enabled in servers",
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn only_for_dm(ctx: &mut Context, msg: &Message) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        "I'm sorry, but that command is only enabled in dms",
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn lacking_permissions(ctx: &mut Context, msg: &Message, permissions: Permissions) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        &format!("I lack the permissions `{:?}` to do that", permissions),
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn too_many_arguments(ctx: &mut Context, msg: &Message, max: u16, given: usize) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        &format!(
            "I'm sorry, but this command takes at most {} arguments, but you gave {}",
            max, given
        ),
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn not_enough_arguments(ctx: &mut Context, msg: &Message, min: u16, given: usize) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        &format!(
            "I'm sorry, but this command requires at least {} arguments, but you gave {}",
            min, given
        ),
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn blocked_user(_ctx: &mut Context, msg: &Message) {
    info!(
        "Blocked user `{}#{} ({})` attempted to use bot",
        msg.author.name, msg.author.discriminator, msg.author.id
    );
}

#[inline]
fn blocked_guild(_ctx: &mut Context, msg: &Message) {
    info!(
        "User `{}#{} ({})` attempted to use bot in blocked guild {}",
        msg.author.name,
        msg.author.discriminator,
        msg.author.id,
        if let Some(id) = msg.guild_id {
            format!("`{}`", id)
        } else {
            "".to_string()
        }
    );
}

#[inline]
fn command_disabled(ctx: &mut Context, msg: &Message, message: &str) {
    if let Err(err) = msg
        .channel_id
        .say(&ctx.http, "I'm sorry, but that command is disabled")
    {
        error!("Message Error: {:?}", err);
    }
    info!("{:?}", message);
}

#[inline]
fn ratelimited(ctx: &mut Context, msg: &Message, seconds: i64) {
    if let Err(err) = msg.channel_id.say(
        &ctx.http,
        &format!("Try this again in {} seconds.", seconds),
    ) {
        error!("Message Error: {:?}", err);
    }
}

#[inline]
fn check_failed(ctx: &mut Context, msg: &Message, message: &'static str, reason: &Reason) {
    use serenity::framework::standard::Reason::*;

    let reason_message = match reason {
        Unknown => "That command doesn't exist, try".to_string(),
        User(string) => format!("You're not allowed to use that command because {}", string),
        Log(string) => {
            warn!("{}", string);
            "You're not allowed to use that command".to_string()
        }
        UserAndLog { user, log } => {
            warn!("{}", log);
            format!("You're not allowed to use that command because {}", user)
        }
        _ => "You're not allowed to use that command".to_string(),
    };

    if let Err(err) = msg.channel_id.say(&ctx.http, &reason_message) {
        error!("Message Error: {:?}", err);
    }

    error!("Check Failed: {:?} | {:?}", message, reason);
}

#[inline]
pub fn after_command(
    ctx: &mut Context,
    msg: &Message,
    string: &str,
    result: Result<(), CommandError>,
) {
    use crate::inserts::DatabaseKey;

    if let Err(err) = result {
        error!("Command Error: {:?}", err);
    }

    let data = ctx.data.read();

    if let Some(database) = data.get::<DatabaseKey>() {
        // Update database
    }
}
