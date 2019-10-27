use crate::commands::checks::{CAN_BAN_CHECK, CAN_KICK_CHECK};
use log::error;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandError, CommandResult,
    },
    model::{channel::Message, error::Error as ModelError, guild::BanOptions, id::UserId},
    prelude::HttpError::UnsuccessfulRequest,
    Error,
};

group!({
    name: "admin",
    options: {
        prefixes: ["sudo"],
    },
    commands: [kick, ban]
});

#[command]
#[checks(can_kick)]
#[only_in(guilds)]
#[description("Kick members from the server")]
#[usage("<member>")]
#[min_args(1)]
#[max_args(1)]
fn kick(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.mentions.is_empty() {
        return Err(CommandError::from(
            "You have to mention a member to kick them",
        ));
    }

    let member = &msg.mentions[0];
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx, member) {
            {
                // Circumvents borrow rules
                let member_id = UserId(member.user.read().id.0);

                if member_id == msg.author.id {
                    if let Err(err) = msg.channel_id.say(&ctx, "I can't let you kick yourself!") {
                        error!("Message Error: {:?}", err);
                    }
                    return Ok(());
                } else if member_id == ctx.cache.read().user.id {
                    if let Err(err) = msg.channel_id.say(&ctx, "I refuse to kick myself!") {
                        error!("Message Error: {:?}", err);
                    }
                    return Ok(());
                }
            }

            match member.kick(&ctx) {
                    Ok(()) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Kicked {}#{}", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::GuildNotFound)) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("{}#{} was not found in this server", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("I don't have enough permissions to kick {}#{}. Try giving me the following permissions: `{:?}`", member.user.read().name, member.user.read().discriminator, missing_perms),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::Hierarchy)) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Failed to kick {}#{}, they have a higher role than I do, try lowering them or boosting me", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    err => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Failed to kick {}#{}", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                        error!("Kick Error: {:?}", err)
                    }
                }
        } else if let Err(err) = msg.reply(
            &ctx,
            &format!(
                "{}#{} is not in this guild!",
                member.name, member.discriminator
            ),
        ) {
            error!("Message Error: {:?}", err);
        }
    } else if let Err(err) = msg.reply(&ctx, "There was an error kicking, try again later") {
        error!("Message Error: {:?}", err);
    }

    Ok(())
}

struct InternBanOptions {
    days_to_delete: u8,
    reason: String,
}

impl InternBanOptions {
    const fn new(days_to_delete: u8, reason: String) -> Self {
        Self {
            days_to_delete,
            reason,
        }
    }
}

impl BanOptions for InternBanOptions {
    fn dmd(&self) -> u8 {
        self.days_to_delete
    }

    fn reason(&self) -> &str {
        &self.reason
    }
}

#[command]
#[checks(can_ban)]
#[only_in(guilds)]
#[description("Ban members from the server")]
#[usage("<members> \"reason\" <days of messages to delete>")]
#[min_args(1)]
#[max_args(3)]
fn ban(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.mentions.is_empty() {
        return Err(CommandError::from(
            "You have to mention a member to ban them",
        ));
    }

    args.advance(); // Skip the user mention
    let reason = match args.single_quoted::<String>() {
        Ok(reason) => reason,
        Err(_) => String::from(""),
    };

    let days_to_delete = match args.parse::<u8>() {
        Ok(days) => {
            if days >= 7 {
                7
            } else {
                days
            }
        }
        Err(_) => 0,
    };

    let member = &msg.mentions[0];
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx, member) {
            {
                let member_id = UserId(member.user.read().id.0);

                if member_id == msg.author.id {
                    if let Err(err) = msg.channel_id.say(&ctx, "I can't let you ban yourself!") {
                        error!("Message Error: {:?}", err);
                    }
                    return Ok(());
                } else if member_id == ctx.cache.read().user.id {
                    if let Err(err) = msg.channel_id.say(&ctx, "I refuse to ban myself!") {
                        error!("Message Error: {:?}", err);
                    }
                    return Ok(());
                }
            }

            match member.ban(&ctx, &InternBanOptions::new(days_to_delete, reason)) {
                    Ok(()) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Banned {}#{}", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::GuildNotFound)) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("{}#{} was not found in this server", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::InvalidPermissions(missing_perms))) =>
                    {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("I don't have enough permissions to ban {}#{}. Try giving me the following permissions: `{:?}`", member.user.read().name, member.user.read().discriminator, missing_perms),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    Err(Error::Model(ModelError::Hierarchy)) => {
                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Failed to ban {}#{}, they have a higher role than I do, try lowering them or boosting me", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                    }
                    err => {
                        // This is really hacky, but it's the only way I know to get down through the layers in order to deref and get what is needed
                        if let Err(Error::Http(ref request)) = err {
                            if let UnsuccessfulRequest(ref response) = **request {
                                if response.error.message == "Missing Permissions" {
                                    if let Err(err) = msg.channel_id.say(
                                        &ctx,
                                        &format!("I don't have enough permissions to ban {}#{}. Try giving me the `BAN_MEMBERS` permission", member.user.read().name, member.user.read().discriminator),
                                    ) {
                                        error!("Message Error: {:?}", err);
                                    }
                                    return Ok(());
                                }
                            }
                        }

                        if let Err(err) = msg.channel_id.say(
                            &ctx,
                            &format!("Failed to ban {}#{}", member.user.read().name, member.user.read().discriminator),
                        ) {
                            error!("Message Error: {:?}", err);
                        }
                        error!("Ban Error: {:?}", err);
                    }
                }
        } else if let Err(err) = msg.reply(
            &ctx,
            &format!(
                "{}#{} is not in this guild!",
                member.name, member.discriminator
            ),
        ) {
            error!("Message Error: {:?}", err);
        }
    } else if let Err(err) = msg.reply(&ctx, "There was an error banning, try again later") {
        error!("Message Error: {:?}", err);
    }

    Ok(())
}
