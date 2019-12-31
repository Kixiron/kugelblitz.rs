use crate::commands::command_prelude::*;

#[command]
pub fn online(context: &mut Context, _msg: &Message) -> CommandResult {
    context.online();
    Ok(())
}

#[command]
pub fn idle(context: &mut Context, _msg: &Message) -> CommandResult {
    context.idle();
    Ok(())
}

#[command]
pub fn dnd(context: &mut Context, _msg: &Message) -> CommandResult {
    context.dnd();
    Ok(())
}

#[command]
pub fn invisible(context: &mut Context, _msg: &Message) -> CommandResult {
    context.invisible();
    Ok(())
}

#[command]
#[aliases("resetstatus")]
pub fn reset_status(context: &mut Context, _msg: &Message) -> CommandResult {
    context.reset_presence();
    Ok(())
}
