use crate::commands::command_prelude::*;
use std::{fs::File, io::copy};

#[command]
#[description = "(Un)sets the bots' avatar. Takes a url, nothing, or an attachment."]
#[usage = "[<avatar_url>]"]
#[example = "https://memeguy.com/photos/images/i-think-i-found-my-favorite-stock-photo-129777.jpg"]
pub fn avatar(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut p = serenity::builder::EditProfile::default();
    if !msg.attachments.is_empty() {
        let url = &msg
            .attachments
            .get(0)
            .ok_or_else(|| "Failed to get attachment")?
            .url;

        let tmpdir = tempfile::tempdir()?;
        let mut response = reqwest::blocking::get(url)?;

        let (mut out_file, out_path) = {
            let filename = response
                .url()
                .path_segments()
                .and_then(|seg| seg.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or_else(|| "Failed to get filename from url.")?;

            let filename = tmpdir.path().join(filename);

            (File::create(filename.clone())?, filename)
        };

        copy(&mut response, &mut out_file)?;

        let base64 = serenity::utils::read_image(out_path)?;
        p.avatar(Some(&base64));

        let map = serenity::utils::hashmap_to_json_map(p.0);
        ctx.http.edit_profile(&map)?;
    } else if args.is_empty() {
        p.avatar(None);

        let map = serenity::utils::hashmap_to_json_map(p.0);
        ctx.http.edit_profile(&map)?;
    } else {
        let url = args.single::<String>()?;
        let tmpdir = tempfile::tempdir()?;

        let mut response = reqwest::blocking::get(&url)?;

        let (mut out_file, out_path) = {
            let filename = response
                .url()
                .path_segments()
                .and_then(|seg| seg.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .ok_or_else(|| "Failed to get filename from url.")?;

            let filename = tmpdir.path().join(filename);

            (File::create(filename.clone())?, filename)
        };

        copy(&mut response, &mut out_file)?;

        let base64 = serenity::utils::read_image(out_path)?;
        p.avatar(Some(&base64));

        let map = serenity::utils::hashmap_to_json_map(p.0);
        ctx.http.edit_profile(&map)?;
    }

    msg.react(&ctx, "✅")?;

    Ok(())
}
