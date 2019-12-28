use log::info;
use serenity::{
    client::{Context, EventHandler},
    model::{gateway::Ready, id::GuildId},
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, ready: Ready) {
        info!(
            "Connected to discord as {} on {}",
            ready.user.tag(),
            chrono::Utc::now().format("%c")
        );
    }

    fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        info!("Fully loaded the cache");

        let cache = ctx.cache.read();
        let map = {
            let mut map = std::collections::HashMap::with_capacity(8);

            map.insert("Users", cache.users.len());
            map.insert("Channels", cache.channels.len());
            map.insert("Direct Message", cache.private_channels.len());
            map.insert("Groups", cache.groups.len());
            map.insert("Servers", guilds.len());
            map.insert("Unavailable Guilds", cache.unavailable_guilds.len());
            map.insert("Shards", cache.shard_count as usize);

            map
        };
        let tag = cache.user.tag();
        let args = format_args!("{} is running", tag);

        let record = log::Record::builder()
            .args(args)
            .level(log::Level::Info)
            .key_values(&map)
            .build();
        log::logger().log(&record);
    }
}
