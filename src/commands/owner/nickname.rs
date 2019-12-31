use crate::commands::command_prelude::*;

#[command]
#[description = "Changes the bots' nickname for the current guild."]
#[usage = "\"[<Nickname>]\""]
#[example = "\"Kugelblitz\""]
#[only_in(guilds)]
fn nickname(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        if let Some(guild_id) = msg.guild_id {
            ctx.http
                .edit_nickname(guild_id.0, None)
                .map_err(|e| CommandError(e.to_string()))?
        }
    } else {
        let nick = args.single_quoted::<String>()?;

        if let Some(guild_id) = msg.guild_id {
            ctx.http
                .edit_nickname(guild_id.0, Some(&nick))
                .map_err(|e| CommandError(e.to_string()))?
        }
    }

    Ok(())
}
