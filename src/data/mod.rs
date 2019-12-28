use serenity::client::Client;

mod data_prelude {
    pub use parking_lot::{Mutex, RwLock};
    pub use serenity::prelude::TypeMapKey;
    pub use std::sync::Arc;
}
mod database;
mod shard_manager;
mod task_scheduler;

pub use database::*;
pub use shard_manager::*;
pub use task_scheduler::*;

pub fn setup_data(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    use data_prelude::*;

    let mut data = client.data.write();

    data.insert::<database::DatabaseContainer>(Arc::new(RwLock::new(database::Database::new()?)));
    data.insert::<shard_manager::ShardManagerContainer>(Arc::clone(&client.shard_manager));

    Ok(())
}
