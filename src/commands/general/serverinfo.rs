use crate::commands::command_prelude::*;

#[command]
#[description = "Get information about the current server"]
#[only_in(guilds)]
pub fn serverinfo(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = msg
        .guild_id
        .ok_or("Should be a guild because only_in(guilds) is enforced")?;

    let partial_guild = guild_id.to_partial_guild(&ctx)?;
    let (num_members, num_bots) = if let Some(guild) = guild_id.to_guild_cached(&ctx) {
        let (mut members, mut bots) = (0, 0);
        for member in guild.read().members.values() {
            if member.user.read().bot {
                bots += 1
            } else {
                members += 1
            }
        }
        (members, bots)
    } else {
        let (mut members, mut bots) = (0, 0);
        for member in partial_guild.members(&ctx, None, None)? {
            if member.user.read().bot {
                bots += 1
            } else {
                members += 1
            }
        }
        (members, bots)
    };

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.description("Server Info")
                .field(
                    "Members",
                    format!(
                        "{} ({} users, {} bots)",
                        num_members + num_bots,
                        num_members,
                        num_bots
                    ),
                    false,
                )
                .field("Roles", partial_guild.roles.len(), true)
                .field("Emojis", partial_guild.emojis.len(), true)
                .field("Region", partial_guild.region, true)
        })
    })?;

    Ok(())
}
