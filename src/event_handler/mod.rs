use log::info;
use serenity::{
    client::{Context, EventHandler},
    model::{gateway::Ready, id::GuildId},
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        if let Some([shard_id, total_shards]) = ready.shard {
            info!(
                "Shard No. {} out of {} connected",
                shard_id + 1,
                total_shards
            );
        } else {
            info!("Shard No. {} connected", ctx.shard_id + 1);
        }
    }

    fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Fully loaded the discord cache");

        let (tag, map) = {
            let cache = ctx.cache.read();

            let tag = cache.user.tag();
            let map = {
                let mut map = std::collections::HashMap::with_capacity(8);

                map.insert("Users", cache.users.len());
                map.insert("Channels", cache.channels.len());
                map.insert("Direct Messages", cache.private_channels.len());
                map.insert("Groups", cache.groups.len());
                map.insert("Servers", guilds.len());
                map.insert("Unavailable Guilds", cache.unavailable_guilds.len());
                map.insert("Shards", cache.shard_count as usize);

                map
            };

            (tag, map)
        };

        log::logger().log(
            &log::Record::builder()
                .args(format_args!(
                    "{} started at {}",
                    tag,
                    chrono::Utc::now().format("%c")
                ))
                .target("kugelblitz")
                .module_path_static(Some("event_handler"))
                .file_static(Some(file!()))
                .line(Some(line!()))
                .level(log::Level::Info)
                .key_values(&map)
                .build(),
        );
    }
}
