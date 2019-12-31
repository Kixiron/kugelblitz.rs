use crate::commands::command_prelude::*;

#[command]
#[description = "Set the bots' name"]
#[usage = "\"<Username>\""]
#[example = "\"Kugelblitz\""]
#[min_args(1)]
pub fn name(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = args.single_quoted::<String>()?;

    {
        let mut cache = ctx.cache.write();
        cache.user.edit(&ctx, |user| user.username(&name))?;
    }

    info!(
        "{} ({}) set the name to '{}'",
        msg.author.tag(),
        msg.author.id,
        name
    );

    msg.react(&ctx, "✅")?;

    Ok(())
}
