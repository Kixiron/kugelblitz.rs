use super::data_prelude::*;
use serenity::client::bridge::gateway::ShardManager;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
