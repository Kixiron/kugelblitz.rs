use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CheckResult, CommandOptions},
    model::channel::Message,
};

#[check]
#[name = "can_kick"]
#[check_in_help(true)]
fn can_kick(
    ctx: &mut Context,
    msg: &Message,
    _args: &mut Args,
    _cmd: &CommandOptions,
) -> CheckResult {
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx, msg.author.id) {
            if let Ok(permissions) = member.permissions(&ctx) {
                return permissions.kick_members().into();
            }
        }
    }

    false.into()
}

#[check]
#[name = "can_ban"]
#[check_in_help(true)]
fn can_ban(
    ctx: &mut Context,
    msg: &Message,
    _args: &mut Args,
    _cmd: &CommandOptions,
) -> CheckResult {
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx, msg.author.id) {
            if let Ok(permissions) = member.permissions(&ctx) {
                return permissions.ban_members().into();
            }
        }
    }

    false.into()
}
