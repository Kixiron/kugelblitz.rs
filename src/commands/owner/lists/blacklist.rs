use crate::commands::command_prelude::*;
use serenity::framework::standard::macros::group;
use std::time::Instant;

#[group]
#[commands(user, server)]
pub struct Blacklist;

#[command]
#[description = "Blacklist users"]
#[usage = "{<User>}"]
#[min_args(1)]
pub fn user(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let (mut users_to_blacklist, mut user_strings) = (Vec::new(), Vec::new());
    {
        if msg.mentions.len() == 0 {
            let id = args.single::<u64>()?;
            users_to_blacklist.push(id);
            user_strings.push(id.to_string());

            while let Ok(id) = args.single::<u64>() {
                users_to_blacklist.push(id);
                user_strings.push(id.to_string());
            }
        } else {
            for user in &msg.mentions {
                users_to_blacklist.push(*user.id.as_u64());
                user_strings.push(user.tag());
            }
        }
    }
    let user_strings = user_strings.join(", ");

    let confirm = msg.channel_id.send_message(&ctx, |m| {
        m.content(format!(
            "Are you sure you want to blacklist {}?",
            user_strings
        ))
        .reactions(vec!["✅"])
    })?;
    let start = Instant::now();

    users_to_blacklist.sort_unstable();

    while start.elapsed().as_secs() <= 10 {
        if let Ok(users) = confirm.reaction_users(&ctx, "✅", Some(100), None) {
            if users.iter().find(|u| u.id == msg.author.id).is_some() {
                msg.channel_id
                    .say(&ctx, format!("Ok, blacklisting {}", user_strings))?;

                let data = ctx.data.read();
                if let Some(users) = data.get::<crate::data::UserBlacklistContainer>() {
                    if users_to_blacklist.len() == 1 {
                        let user = users_to_blacklist[0];
                        let mut users = users.write();

                        if let Err(index) = users.binary_search(&user) {
                            users.insert(index, user);
                            info!("Blacklisted {}", user);
                        }
                    } else {
                        let mut users = users.write();

                        let start = match users.binary_search(&users_to_blacklist[0]) {
                            Ok(idx) => idx,
                            Err(idx) => idx,
                        };
                        let end = users.split_off(start);

                        users.extend_from_slice(&users_to_blacklist);
                        users.extend_from_slice(&end);

                        for id in &users_to_blacklist {
                            info!("Blacklisted {}", id);
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
                        config
                            .blacklisted_users
                            .extend_from_slice(&users_to_blacklist);
                        config.blacklisted_users.dedup();

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
#[description = "Blacklist the current server or a server by its id"]
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
                "You must either mention a server or be in one to blacklist it",
            )?;
            return Err(CommandError("No server id".to_string()));
        }
    };

    let confirm = msg.channel_id.send_message(&ctx, |m| {
        m.content(format!("Are you sure you want to blacklist {}?", guild_id))
            .reactions(vec!["✅"])
    })?;
    let start = Instant::now();

    while start.elapsed().as_secs() <= 10 {
        if let Ok(users) = confirm.reaction_users(&ctx, "✅", Some(100), None) {
            if users.iter().find(|u| u.id == msg.author.id).is_some() {
                msg.channel_id
                    .say(&ctx, format!("Ok, blacklisting {}", guild_id))?;

                let data = ctx.data.read();
                if let Some(guilds) = data.get::<crate::data::ServerBlacklistContainer>() {
                    {
                        let mut guilds = guilds.write();

                        if let Err(index) = guilds.binary_search(&guild_id) {
                            guilds.insert(index, guild_id);
                            info!("Blacklisted {}", guild_id);
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
                        if let Err(idx) = config.blacklisted_servers.binary_search(&guild_id) {
                            config.blacklisted_servers.insert(idx, guild_id);

                            blacklist.set_len(0)?;
                            blacklist.seek(SeekFrom::Start(0))?;
                            blacklist.write_all(toml::to_string(&config)?.as_bytes())?;
                        }

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
