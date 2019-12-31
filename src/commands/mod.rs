use crate::data;
use log::{error, info, warn};
use serenity::{
    framework::standard::{DispatchError, StandardFramework},
    model::id::UserId,
};

mod command_prelude {
    pub use crate::commands::utils;
    pub use log::{error, info, warn};
    pub use serenity::{
        client::{bridge::gateway::ShardId, Context},
        framework::standard::{macros::command, Args, CommandError, CommandResult},
        model::{
            channel::Message,
            id::{ChannelId, GuildId, UserId},
        },
        utils::MessageBuilder,
    };
}

mod general;
mod help;
mod owner;
pub mod utils;

pub fn setup_framework(
    owners: std::collections::HashSet<UserId>,
    bot_id: UserId,
) -> StandardFramework {
    StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("*")
                .delimiters(vec![", ", ",", " "])
                .owners(owners)
        })
        .before(|ctx, msg, command_name| {
            // Check if the sending guild is blacklisted
            if let Some(guild_id) = msg.guild_id {
                let data = ctx.data.read();
                if let Some(servers) = data.get::<data::ServerBlacklistContainer>() {
                    if servers.read().binary_search(guild_id.as_u64()).is_ok()
                        && msg.author.id != *crate::OWNER_ID.read()
                    {
                        info!(
                            "User {} ({}) attempted to use command '{}' in blacklisted guild {}",
                            msg.author.tag(),
                            msg.author.id,
                            command_name,
                            if let Ok(partial_guild) = guild_id.to_partial_guild(&ctx) {
                                format!("{} ({})", partial_guild.name, guild_id)
                            } else {
                                guild_id.to_string()
                            }
                        );

                        return false;
                    }
                }
            }

            // Check if the sending user is blacklisted
            {
                let data = ctx.data.read();
                if let Some(users) = data.get::<data::UserBlacklistContainer>() {
                    if users.read().binary_search(msg.author.id.as_u64()).is_ok()
                        && msg.author.id != *crate::OWNER_ID.read()
                    {
                        info!(
                            "Blacklisted user {} ({}) attempted to use command '{}' in {}",
                            msg.author.tag(),
                            msg.author.id,
                            command_name,
                            if let Some(guild_id) = msg.guild_id {
                                if let Ok(partial_guild) = guild_id.to_partial_guild(&ctx) {
                                    format!("{} ({})", partial_guild.name, guild_id)
                                } else {
                                    guild_id.to_string()
                                }
                            } else {
                                "direct messages".to_string()
                            }
                        );

                        return false;
                    }
                }
            }

            true
        })
        .after(|ctx, _msg, command_name, error| {
            if let Err(err) = error {
                error!(target: command_name, "{}", err.0);
            }

            let data = ctx.data.write();
            let db = if let Some(db) = data.get::<data::DatabaseContainer>() {
                db
            } else {
                error!("Attempted to get database for command increments");
                return;
            };

            let tree = match db.read().open_tree("command_counter") {
                Ok(tree) => tree,
                Err(err) => {
                    error!("Failed to open counter tree: {:?}", err);
                    return;
                }
            };

            if let Err(err) = tree.fetch_and_update(command_name, data::Database::increment) {
                error!(
                    "Failed to increment the counter for {}: {:?}",
                    command_name, err
                );
            }
        })
        .unrecognised_command(|ctx, msg, unknown_command_name| {
            if let Err(err) = msg.channel_id.say(
                &ctx,
                format!(
                    "I'm sorry, but I don't have a command called '{}'",
                    unknown_command_name
                ),
            ) {
                error!(target: "Unrecognized Command", "{:?}", err);
            }
        })
        .on_dispatch_error(|ctx, msg, error| match error {
            DispatchError::CheckFailed(string, reason) => {}
            DispatchError::CommandDisabled(name) => {
                if let Err(err) = msg
                    .channel_id
                    .say(&ctx, format!("I'm sorry, but {} is disabled", name))
                {
                    error!("Message Send Error: {:?}", err);
                }
            }
            DispatchError::BlockedUser
            | DispatchError::BlockedGuild
            | DispatchError::BlockedChannel => {}

            DispatchError::OnlyForDM => {}
            DispatchError::OnlyForGuilds => {}
            DispatchError::OnlyForOwners => {}
            DispatchError::LackingRole => {}
            DispatchError::LackingPermissions(permissions) => {}
            DispatchError::NotEnoughArguments { min, given } => {
                if let Err(err) = msg.channel_id.say(
                    &ctx,
                    format!("Need {} arguments, but only got {}.", min, given),
                ) {
                    error!("Message Send Error: {:?}", err);
                }
            }
            DispatchError::TooManyArguments { max, given } => {
                if let Err(err) = msg.channel_id.say(
                    &ctx,
                    format!("Max arguments allowed is {}, but got {}.", max, given),
                ) {
                    error!("Message Send Error: {:?}", err);
                }
            }
            DispatchError::Ratelimited(timeout) => {
                warn!("Ratelimited for {} seconds", timeout);
            }
            DispatchError::IgnoredBot | DispatchError::WebhookAuthor => {}
            err => error!("Unknown dispatch error: {:?}", err),
        })
        .normal_message(|_ctx, _message| {})
        .help(&help::MY_HELP)
        .group(&general::GENERAL_GROUP)
        .group(&owner::OWNER_GROUP)
}
