use crate::commands::command_prelude::*;
use std::time::Instant;

#[command]
#[sub_commands(user, server)]
pub fn unblacklist(_ctx: &mut Context, _msg: &Message) -> CommandResult {
    Ok(())
}

#[command]
#[description = "Blacklist users"]
#[usage = "{<User>}"]
#[min_args(1)]
pub fn user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let (mut users_to_unblacklist, mut user_strings) = (Vec::new(), Vec::new());
    {
        if msg.mentions.len() == 0 {
            let id = args.single::<u64>()?;
            users_to_unblacklist.push(id);
            user_strings.push(id.to_string());

            while let Ok(id) = args.single::<u64>() {
                users_to_unblacklist.push(id);
                user_strings.push(id.to_string());
            }
        } else {
            for user in &msg.mentions {
                users_to_unblacklist.push(*user.id.as_u64());
                user_strings.push(user.tag());
            }
        }
    }
    let user_strings = user_strings.join(", ");

    let confirm = msg.channel_id.send_message(&ctx, |m| {
        m.content(format!(
            "Are you sure you want to unblacklist {}?",
            user_strings
        ))
        .reactions(vec!["✅"])
    })?;
    let start = Instant::now();

    users_to_unblacklist.sort_unstable();

    while start.elapsed().as_secs() <= 10 {
        if let Ok(users) = confirm.reaction_users(&ctx, "✅", Some(100), None) {
            if users.iter().find(|u| u.id == msg.author.id).is_some() {
                msg.channel_id
                    .say(&ctx, format!("Ok, unblacklisting {}", user_strings))?;

                let data = ctx.data.read();
                if let Some(users) = data.get::<crate::data::UserBlacklistContainer>() {
                    {
                        let mut users = users.write();

                        for id in &users_to_unblacklist {
                            if let Some(idx) = (*users).iter().position(|i| i == id) {
                                users.remove(idx);
                                info!("Unblacklisted {}", id);
                            }
                        }
                    }

                    {
                        use crate::config::Config;
                        use std::{
                            fs::OpenOptions,
                            io::{Read, Seek, SeekFrom, Write},
                        };

                        let mut blacklist = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open("./Config.toml")?;
                        let mut content = if let Ok(meta) = blacklist.metadata() {
                            String::with_capacity(meta.len() as usize)
                        } else {
                            String::new()
                        };
                        blacklist.read_to_string(&mut content)?;

                        let mut config: Config = toml::from_str(&content)?;
                        config.blacklisted_users = config
                            .blacklisted_users
                            .into_iter()
                            .filter(|s| !users_to_unblacklist.contains(s))
                            .collect();

                        blacklist.set_len(0)?;
                        blacklist.seek(SeekFrom::Start(0))?;
                        blacklist.write_all(dbg!(toml::to_string(&config))?.as_bytes())?;

                        info!("Updated Config.toml with new blacklisted users");
                    }

                    return Ok(());
                } else {
                    msg.channel_id
                        .say(&ctx, "Something went wrong, try again later")?;

                    return Err(CommandError(
                        "Failed to get UserBlacklistContainer".to_string(),
                    ));
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    msg.channel_id
        .say(&ctx, "You took to much time to respond, canceling")?;

    Ok(())
}

#[command]
#[description = "Unblacklist the current server or a server by its id"]
#[usage = "[<Server Id>]"]
#[example = "0123456789"]
pub fn server(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = if let Ok(guild_id) = args.single::<u64>() {
        guild_id
    } else {
        if let Some(guild_id) = msg.guild_id {
            *guild_id.as_u64()
        } else {
            msg.channel_id.say(
                &ctx,
                "You must either mention a server or be in one to unblacklist it",
            )?;
            return Err(CommandError("No server id".to_string()));
        }
    };

    let confirm = msg.channel_id.send_message(&ctx, |m| {
        m.content(format!(
            "Are you sure you want to unblacklist {}?",
            guild_id
        ))
        .reactions(vec!["✅"])
    })?;
    let start = Instant::now();

    while start.elapsed().as_secs() <= 10 {
        if let Ok(users) = confirm.reaction_users(&ctx, "✅", Some(100), None) {
            if users.iter().find(|u| u.id == msg.author.id).is_some() {
                msg.channel_id
                    .say(&ctx, format!("Ok, unblacklisting {}", guild_id))?;

                let data = ctx.data.read();
                if let Some(guilds) = data.get::<crate::data::ServerBlacklistContainer>() {
                    {
                        let mut guilds = guilds.write();

                        if let Ok(index) = guilds.binary_search(&guild_id) {
                            guilds.remove(index);
                            info!("Unblacklisted {}", guild_id);
                        }
                    }

                    {
                        use crate::config::Config;
                        use std::{
                            fs::OpenOptions,
                            io::{Read, Seek, SeekFrom, Write},
                        };

                        let mut blacklist = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open("./Config.toml")?;
                        let mut content = if let Ok(meta) = blacklist.metadata() {
                            String::with_capacity(meta.len() as usize)
                        } else {
                            String::new()
                        };
                        blacklist.read_to_string(&mut content)?;

                        let mut config: Config = toml::from_str(&content)?;
                        config.blacklisted_servers = config
                            .blacklisted_servers
                            .into_iter()
                            .filter(|s| *s != guild_id)
                            .collect();

                        blacklist.set_len(0)?;
                        blacklist.seek(SeekFrom::Start(0))?;
                        blacklist.write_all(toml::to_string(&config)?.as_bytes())?;

                        info!("Updated Config.toml with new blacklisted servers");
                    }

                    return Ok(());
                } else {
                    msg.channel_id
                        .say(&ctx, "Something went wrong, try again later")?;

                    return Err(CommandError(
                        "Failed to get ServerBlacklistContainer".to_string(),
                    ));
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    msg.channel_id
        .say(&ctx, "You took to much time to respond, canceling")?;

    Ok(())
}
