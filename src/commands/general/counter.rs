use crate::commands::command_prelude::*;
use inflector::cases::titlecase::to_title_case;
use std::convert::TryInto;

#[command]
#[aliases("commandcounter")]
#[description = "See the usage of various commands"]
pub fn command_counter(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.write();
    let db = if let Some(db) = data.get::<crate::data::DatabaseContainer>() {
        db
    } else {
        return Err(CommandError("Failed to get Database".to_string()));
    };

    let tree = match db.read().open_tree("command_counter") {
        Ok(tree) => tree,
        Err(err) => {
            return Err(CommandError(format!(
                "Failed to open command_counter: {:?}",
                err
            )));
        }
    };

    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.description("Command Usage")
                .fields(
                    tree.into_iter()
                        .flat_map(Result::ok)
                        .filter_map(|(name, count)| {
                            let count: Result<[u8; 8], _> = count.as_ref().try_into();

                            let field = (
                                to_title_case(std::str::from_utf8(name.as_ref()).ok()?),
                                u64::from_be_bytes(count.ok()?),
                                true,
                            );

                            if field.1 != 0 {
                                Some(field)
                            } else {
                                None
                            }
                        }),
                )
                .colour(0x00FFEB)
        })
    })?;

    Ok(())
}
