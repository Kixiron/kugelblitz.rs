use serenity::client::Client;

mod data_prelude {
    pub use parking_lot::{Mutex, RwLock};
    pub use serenity::prelude::TypeMapKey;
    pub use std::sync::Arc;
}

mod database;
mod dispatch;
mod lists;
mod shard_manager;
mod task_scheduler;

pub use database::*;
pub use dispatch::*;
pub use lists::*;
pub use shard_manager::*;
pub use task_scheduler::*;

pub fn setup_data(
    client: &mut Client,
    config: crate::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    use data_prelude::*;

    let mut data = client.data.write();

    data.insert::<database::DatabaseContainer>(Arc::new(RwLock::new(database::Database::new()?)));
    data.insert::<shard_manager::ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<task_scheduler::SchedulerContainer>(
        task_scheduler::SchedulerContainer::new_scheduler(),
    );
    data.insert::<dispatch::DispatchContainer>(dispatch::DispatchContainer::new_dispatcher());

    let (blacklisted_users, blacklisted_servers) = blacklists_from_config(config);
    data.insert::<lists::UserBlacklistContainer>(blacklisted_users);
    data.insert::<lists::ServerBlacklistContainer>(blacklisted_servers);

    Ok(())
}
