use crate::commands::command_prelude::*;

#[command]
#[description = "Get the avatar of a user or yourself"]
#[usage = "[<User>]"]
#[example = "@Someone"]
pub fn avatar(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = utils::get_mentioned_user_or_sender(ctx, msg, &mut args)?;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let avatar_url = utils::avatar_url(&user);

            e.title("Avatar")
                .image(&avatar_url)
                .author(|a| a.icon_url(avatar_url).name(user.tag()))
        })
    })?;

    Ok(())
}
