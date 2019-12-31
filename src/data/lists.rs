use super::data_prelude::*;
use crate::config::Config;

pub fn blacklists_from_config(config: Config) -> (Arc<RwLock<Vec<u64>>>, Arc<RwLock<Vec<u64>>>) {
    (
        Arc::new(RwLock::new(config.blacklisted_users)),
        Arc::new(RwLock::new(config.blacklisted_servers)),
    )
}

pub struct UserBlacklistContainer;

impl TypeMapKey for UserBlacklistContainer {
    type Value = Arc<RwLock<Vec<u64>>>;
}

pub struct ServerBlacklistContainer;

impl TypeMapKey for ServerBlacklistContainer {
    type Value = Arc<RwLock<Vec<u64>>>;
}
