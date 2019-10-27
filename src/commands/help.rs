use serenity::{
    client::Context,
    framework::standard::{
        help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};
use std::collections::HashSet;

#[help]
#[individual_command_tip = "If you want more information about a specific command, pass the command as argument."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "~"]
#[lacking_permissions = "Strike"]
#[lacking_role = "Strike"]
#[wrong_channel = "Strike"]
#[lacking_ownership = "Strike"]
#[strikethrough_commands_tip_in_guild("If a command is struck through, that means that you are unable to currently use it.\nThis happens for many reasons, like lacking a role, permissions or ownership.")]
#[strikethrough_commands_tip_in_dm("If a command is struck through, that means that you are unable to currently use it.\nSome commands are unable to be used in DMs, and some may be unavailable for other reasons.")]
#[embed_error_colour(RED)]
#[embed_success_colour(BLURPLE)]
fn kugelblitz_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}
