use serenity::{
    client::Context,
    framework::standard::{Args, CommandError},
    model::{channel::Message, id::UserId, user::User},
};

pub fn avatar_url(user: &User) -> String {
    user.avatar_url()
        .unwrap_or_else(|| user.default_avatar_url())
}

pub fn get_mentioned_user_or_sender(
    ctx: &Context,
    msg: &Message,
    args: &mut Args,
) -> Result<User, CommandError> {
    if msg.mentions.len() > 0 {
        Ok(msg.mentions[0].clone())
    } else if let Ok(user_id) = args.single::<u64>() {
        if let Ok(user) = UserId(user_id).to_user(ctx) {
            Ok(user)
        } else {
            msg.channel_id.say(
                &ctx,
                &format!(
                    "I'm sorry, but I couldn't find a user by the id {}",
                    user_id
                ),
            )?;

            Err(CommandError("Invalid User Id".to_string()))
        }
    } else {
        Ok(msg.author.clone())
    }
}

pub fn message_link(msg: &Message) -> Result<String, CommandError> {
    if let Some(guild_id) = msg.guild_id {
        Ok(format!(
            "https://discordapp.com/channels/{guild}/{channel}/{message}",
            guild = guild_id.as_u64(),
            channel = msg.channel_id.as_u64(),
            message = msg.id.as_u64()
        ))
    } else {
        Err(CommandError(
            "Message not received over gateway".to_string(),
        ))
    }
}
