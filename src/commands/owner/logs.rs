use crate::commands::command_prelude::*;

#[command]
#[description = "Uploads the most recent log file"]
#[only_in(dms)]
pub fn logs(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "Uploading file...")?;

    let tempdir = tempfile::tempdir()?;
    let path = tempdir.path().join("latest-log-file.log");

    {
        let file = {
            let mut path = None;
            for log in std::fs::read_dir("./data/logs")? {
                if let Ok(log) = log {
                    if log.file_name().to_string_lossy().ends_with("_rCURRENT.log") {
                        path = Some(log.path());
                        break;
                    }
                }
            }
            path
        }
        .ok_or("There should be a most recent log file")?;

        std::fs::copy(&file, &path)?;
    }

    msg.channel_id
        .send_message(&ctx, |m| m.content("Most recent log file").add_file(&path))?;

    info!(
        "Uploaded most recent log file to {} ({}) on {}",
        msg.author.tag(),
        msg.author.id,
        chrono::offset::Utc::now().format("%c")
    );

    Ok(())
}
