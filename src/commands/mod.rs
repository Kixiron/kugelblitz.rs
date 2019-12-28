use serenity::{framework::standard::StandardFramework, model::id::UserId};

mod general;
mod help;

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
        .before(|_ctx, _msg, _command_name| true)
        .after(|_ctx, _msg, _command_name, _error| {})
        .unrecognised_command(|_ctx, _msg, _unknown_command_name| {})
        .normal_message(|_ctx, _message| {})
        .on_dispatch_error(|_ctx, _msg, _error| {})
        .help(&help::MY_HELP)
        .group(&general::GENERAL_GROUP)
}
